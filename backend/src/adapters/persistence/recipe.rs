//! Recipe persistence.
//!
//! * Listings run two cheap queries (count + one page of cards joined with category and
//!   difficulty names): no N+1, no child rows.
//! * A single recipe is loaded with four indexed queries on one pooled connection.
//! * Writes replace the child rows inside one transaction.

use async_trait::async_trait;
use sqlx::{AssertSqlSafe, FromRow, SqliteConnection};

use super::{SqlitePersistence, contains_pattern, direction_sql, new_id};
use crate::{
    app_error::AppResult,
    entities::{
        page::PageQuery,
        recipe::{
            Ingredient, NamedRef, NutrientValues, Nutrition, Recipe, RecipeDraft, RecipeScope,
            RecipeSort, RecipeSummary, Step,
        },
        taxonomy::TaxonomyItem,
    },
    time::now_iso,
    use_cases::recipe::RecipePersistence,
};

// ---------------------------------------------------------------------------------------------
// Rows
// ---------------------------------------------------------------------------------------------

#[derive(FromRow)]
struct RecipeSummaryDb {
    id: String,
    user_id: String,
    title: String,
    description: Option<String>,
    prep_time: Option<i64>,
    cook_time: Option<i64>,
    servings: i64,
    image_url: Option<String>,
    shared: bool,
    created_at: String,
    updated_at: String,
    category_id: Option<String>,
    category_name: Option<String>,
    difficulty_id: Option<String>,
    difficulty_name: Option<String>,
}

fn named(id: Option<String>, name: Option<String>) -> Option<NamedRef> {
    Some(NamedRef {
        id: id?,
        name: name?,
    })
}

impl From<RecipeSummaryDb> for RecipeSummary {
    fn from(db: RecipeSummaryDb) -> Self {
        RecipeSummary {
            id: db.id,
            user_id: db.user_id,
            category: named(db.category_id, db.category_name),
            difficulty: named(db.difficulty_id, db.difficulty_name),
            title: db.title,
            description: db.description,
            prep_time: db.prep_time,
            cook_time: db.cook_time,
            servings: db.servings,
            image_url: db.image_url,
            shared: db.shared,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

/// A recipe row joined with its (optional) category and difficulty.
#[derive(FromRow)]
struct RecipeDb {
    id: String,
    user_id: String,
    title: String,
    description: Option<String>,
    prep_time: Option<i64>,
    cook_time: Option<i64>,
    servings: i64,
    image_url: Option<String>,
    shared: bool,
    created_at: String,
    updated_at: String,
    c_id: Option<String>,
    c_user_id: Option<String>,
    c_name: Option<String>,
    c_description: Option<String>,
    c_created_at: Option<String>,
    c_updated_at: Option<String>,
    d_id: Option<String>,
    d_user_id: Option<String>,
    d_name: Option<String>,
    d_description: Option<String>,
    d_created_at: Option<String>,
    d_updated_at: Option<String>,
}

fn taxonomy_item(
    id: Option<String>,
    user_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
) -> Option<TaxonomyItem> {
    Some(TaxonomyItem {
        id: id?,
        user_id: user_id?,
        name: name?,
        description,
        created_at: created_at.unwrap_or_default(),
        updated_at: updated_at.unwrap_or_default(),
    })
}

#[derive(FromRow)]
struct StepDb {
    id: i64,
    step_number: i64,
    instructions: String,
    duration: Option<i64>,
}

impl From<StepDb> for Step {
    fn from(db: StepDb) -> Self {
        Step {
            id: db.id,
            step_number: db.step_number,
            instructions: db.instructions,
            duration: db.duration,
        }
    }
}

#[derive(FromRow)]
struct IngredientDb {
    id: i64,
    name: String,
    quantity: Option<f64>,
    unit: Option<String>,
    notes: Option<String>,
    sort_order: i64,
}

impl From<IngredientDb> for Ingredient {
    fn from(db: IngredientDb) -> Self {
        Ingredient {
            id: db.id,
            name: db.name,
            quantity: db.quantity,
            unit: db.unit,
            notes: db.notes,
            sort_order: db.sort_order,
        }
    }
}

#[derive(FromRow)]
struct NutritionDb {
    id: i64,
    serving_size_value: f64,
    serving_size_unit: String,
    calories: Option<f64>,
    protein_g: Option<f64>,
    carbs_g: Option<f64>,
    sugar_g: Option<f64>,
    fat_g: Option<f64>,
    saturated_fat_g: Option<f64>,
    fiber_g: Option<f64>,
    sodium_mg: Option<f64>,
}

impl From<NutritionDb> for Nutrition {
    fn from(db: NutritionDb) -> Self {
        Nutrition {
            id: db.id,
            serving_size_value: db.serving_size_value,
            serving_size_unit: db.serving_size_unit,
            values: NutrientValues {
                calories: db.calories,
                protein_g: db.protein_g,
                carbs_g: db.carbs_g,
                sugar_g: db.sugar_g,
                fat_g: db.fat_g,
                saturated_fat_g: db.saturated_fat_g,
                fiber_g: db.fiber_g,
                sodium_mg: db.sodium_mg,
            },
        }
    }
}

fn sort_sql(sort: RecipeSort) -> &'static str {
    match sort {
        RecipeSort::Title => "r.title COLLATE NOCASE",
        RecipeSort::CreatedAt => "r.created_at",
        RecipeSort::UpdatedAt => "r.updated_at",
        RecipeSort::PrepTime => "r.prep_time",
        RecipeSort::CookTime => "r.cook_time",
        RecipeSort::Servings => "r.servings",
    }
}

// ---------------------------------------------------------------------------------------------
// Port implementation
// ---------------------------------------------------------------------------------------------

#[async_trait]
impl RecipePersistence for SqlitePersistence {
    async fn page_recipes(
        &self,
        scope: &RecipeScope,
        query: &PageQuery<RecipeSort>,
    ) -> AppResult<(Vec<RecipeSummary>, i64)> {
        // ?1 = owner (unused for the shared scope), ?2 = LIKE pattern or NULL.
        let (scope_sql, owner) = match scope {
            RecipeScope::OwnedBy(user_id) => ("r.user_id = ?1", user_id.as_str()),
            RecipeScope::Shared => ("r.shared = 1", ""),
        };
        let filter = format!(
            "{scope_sql} AND (?2 IS NULL OR r.title LIKE ?2 ESCAPE '\\' OR r.description LIKE ?2 ESCAPE '\\')"
        );
        let pattern = query.search.as_deref().map(contains_pattern);

        let total: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM recipes r WHERE {filter}"
        )))
        .bind(owner)
        .bind(pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;
        if total == 0 || query.offset() >= total {
            return Ok((Vec::new(), total));
        }

        let dir = direction_sql(query.direction);
        let rows: Vec<RecipeSummaryDb> = sqlx::query_as(AssertSqlSafe(format!(
            "SELECT r.id, r.user_id, r.title, r.description, r.prep_time, r.cook_time, r.servings,
                    r.image_url, r.shared, r.created_at, r.updated_at,
                    c.id AS category_id, c.name AS category_name,
                    d.id AS difficulty_id, d.name AS difficulty_name
             FROM recipes r
             LEFT JOIN categories   c ON c.id = r.category_id
             LEFT JOIN difficulties d ON d.id = r.difficulty_id
             WHERE {filter}
             ORDER BY {} {dir}, r.id {dir}
             LIMIT ?3 OFFSET ?4",
            sort_sql(query.sort),
        )))
        .bind(owner)
        .bind(pattern.as_deref())
        .bind(query.size)
        .bind(query.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok((rows.into_iter().map(Into::into).collect(), total))
    }

    async fn find_recipe(&self, id: &str) -> AppResult<Option<Recipe>> {
        let mut conn = self.pool.acquire().await?;
        let conn = &mut *conn;

        let Some(row): Option<RecipeDb> = sqlx::query_as(
            "SELECT r.id, r.user_id, r.title, r.description, r.prep_time, r.cook_time, r.servings,
                    r.image_url, r.shared, r.created_at, r.updated_at,
                    c.id AS c_id, c.user_id AS c_user_id, c.name AS c_name, c.description AS c_description,
                    c.created_at AS c_created_at, c.updated_at AS c_updated_at,
                    d.id AS d_id, d.user_id AS d_user_id, d.name AS d_name, d.description AS d_description,
                    d.created_at AS d_created_at, d.updated_at AS d_updated_at
             FROM recipes r
             LEFT JOIN categories   c ON c.id = r.category_id
             LEFT JOIN difficulties d ON d.id = r.difficulty_id
             WHERE r.id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        else {
            return Ok(None);
        };

        let steps: Vec<StepDb> = sqlx::query_as(
            "SELECT id, step_number, instructions, duration FROM recipe_steps
             WHERE recipe_id = ? ORDER BY step_number",
        )
        .bind(id)
        .fetch_all(&mut *conn)
        .await?;

        let ingredients: Vec<IngredientDb> = sqlx::query_as(
            "SELECT id, name, quantity, unit, notes, sort_order FROM recipe_ingredients
             WHERE recipe_id = ? ORDER BY sort_order, id",
        )
        .bind(id)
        .fetch_all(&mut *conn)
        .await?;

        let nutrition: Option<NutritionDb> = sqlx::query_as(
            "SELECT id, serving_size_value, serving_size_unit, calories, protein_g, carbs_g, sugar_g,
                    fat_g, saturated_fat_g, fiber_g, sodium_mg
             FROM recipe_nutrition WHERE recipe_id = ?",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?;

        Ok(Some(Recipe {
            id: row.id,
            user_id: row.user_id,
            category: taxonomy_item(
                row.c_id,
                row.c_user_id,
                row.c_name,
                row.c_description,
                row.c_created_at,
                row.c_updated_at,
            ),
            difficulty: taxonomy_item(
                row.d_id,
                row.d_user_id,
                row.d_name,
                row.d_description,
                row.d_created_at,
                row.d_updated_at,
            ),
            title: row.title,
            description: row.description,
            prep_time: row.prep_time,
            cook_time: row.cook_time,
            servings: row.servings,
            image_url: row.image_url,
            shared: row.shared,
            steps: steps.into_iter().map(Into::into).collect(),
            ingredients: ingredients.into_iter().map(Into::into).collect(),
            nutrition: nutrition.map(Into::into),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    async fn recipe_owner(&self, id: &str) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT user_id FROM recipes WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn create_recipe(&self, user_id: &str, draft: &RecipeDraft) -> AppResult<String> {
        let id = new_id();
        let now = now_iso();
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "INSERT INTO recipes (id, user_id, category_id, difficulty_id, title, description, prep_time,
                                  cook_time, servings, image_url, shared, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(user_id)
        .bind(&draft.category_id)
        .bind(&draft.difficulty_id)
        .bind(&draft.title)
        .bind(&draft.description)
        .bind(draft.prep_time)
        .bind(draft.cook_time)
        .bind(draft.servings)
        .bind(&draft.image_url)
        .bind(draft.shared)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;
        insert_children(&mut tx, &id, draft).await?;
        tx.commit().await?;
        Ok(id)
    }

    async fn update_recipe(&self, id: &str, draft: &RecipeDraft) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE recipes SET category_id = ?, difficulty_id = ?, title = ?, description = ?,
                    prep_time = ?, cook_time = ?, servings = ?, image_url = ?, shared = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&draft.category_id)
        .bind(&draft.difficulty_id)
        .bind(&draft.title)
        .bind(&draft.description)
        .bind(draft.prep_time)
        .bind(draft.cook_time)
        .bind(draft.servings)
        .bind(&draft.image_url)
        .bind(draft.shared)
        .bind(now_iso())
        .bind(id)
        .execute(&mut *tx)
        .await?;

        for table in ["recipe_steps", "recipe_ingredients", "recipe_nutrition"] {
            sqlx::query(AssertSqlSafe(format!(
                "DELETE FROM {table} WHERE recipe_id = ?"
            )))
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }
        insert_children(&mut tx, id, draft).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Children go with it (`ON DELETE CASCADE`).
    async fn delete_recipe(&self, id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM recipes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

async fn insert_children(
    conn: &mut SqliteConnection,
    id: &str,
    draft: &RecipeDraft,
) -> sqlx::Result<()> {
    // Prepared statements are cached per connection, so these loops parse each SQL only once.
    for s in &draft.steps {
        sqlx::query("INSERT INTO recipe_steps (recipe_id, step_number, instructions, duration) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(s.step_number)
            .bind(&s.instructions)
            .bind(s.duration)
            .execute(&mut *conn)
            .await?;
    }
    for i in &draft.ingredients {
        sqlx::query(
            "INSERT INTO recipe_ingredients (recipe_id, name, quantity, unit, notes, sort_order)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(&i.name)
        .bind(i.quantity)
        .bind(&i.unit)
        .bind(&i.notes)
        .bind(i.sort_order)
        .execute(&mut *conn)
        .await?;
    }
    if let Some(n) = &draft.nutrition {
        let v = &n.values;
        sqlx::query(
            "INSERT INTO recipe_nutrition (recipe_id, serving_size_value, serving_size_unit, calories,
                    protein_g, carbs_g, sugar_g, fat_g, saturated_fat_g, fiber_g, sodium_mg)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(n.serving_size_value.unwrap_or(100.0))
        .bind(&n.serving_size_unit)
        .bind(v.calories)
        .bind(v.protein_g)
        .bind(v.carbs_g)
        .bind(v.sugar_g)
        .bind(v.fat_g)
        .bind(v.saturated_fat_g)
        .bind(v.fiber_g)
        .bind(v.sodium_mg)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}
