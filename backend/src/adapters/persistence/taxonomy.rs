use async_trait::async_trait;
use sqlx::{AssertSqlSafe, FromRow};

use super::{SqlitePersistence, contains_pattern, direction_sql, new_id};
use crate::{
    app_error::{AppError, AppResult},
    entities::{
        page::PageQuery,
        taxonomy::{TaxonomyDraft, TaxonomyItem, TaxonomyKind, TaxonomySort},
    },
    time::now_iso,
    use_cases::taxonomy::TaxonomyPersistence,
};

/// Category / difficulty as stored in the database (both tables share this shape).
#[derive(FromRow)]
struct TaxonomyDb {
    id: String,
    user_id: String,
    name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<TaxonomyDb> for TaxonomyItem {
    fn from(db: TaxonomyDb) -> Self {
        TaxonomyItem {
            id: db.id,
            user_id: db.user_id,
            name: db.name,
            description: db.description,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

fn table(kind: TaxonomyKind) -> &'static str {
    match kind {
        TaxonomyKind::Category => "categories",
        TaxonomyKind::Difficulty => "difficulties",
    }
}

fn sort_sql(sort: TaxonomySort) -> &'static str {
    match sort {
        TaxonomySort::Name => "name COLLATE NOCASE",
        TaxonomySort::CreatedAt => "created_at",
        TaxonomySort::UpdatedAt => "updated_at",
    }
}

const COLUMNS: &str = "id, user_id, name, description, created_at, updated_at";

#[async_trait]
impl TaxonomyPersistence for SqlitePersistence {
    async fn page_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        query: &PageQuery<TaxonomySort>,
    ) -> AppResult<(Vec<TaxonomyItem>, i64)> {
        let filter = "user_id = ?1 AND (?2 IS NULL OR name LIKE ?2 ESCAPE '\\' OR description LIKE ?2 ESCAPE '\\')";
        let pattern = query.search.as_deref().map(contains_pattern);

        let total: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM {} WHERE {filter}",
            table(kind)
        )))
        .bind(user_id)
        .bind(pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;
        if total == 0 || query.offset() >= total {
            return Ok((Vec::new(), total));
        }

        let dir = direction_sql(query.direction);
        let rows: Vec<TaxonomyDb> = sqlx::query_as(AssertSqlSafe(format!(
            "SELECT {COLUMNS} FROM {} WHERE {filter} ORDER BY {} {dir}, id {dir} LIMIT ?3 OFFSET ?4",
            table(kind),
            sort_sql(query.sort),
        )))
        .bind(user_id)
        .bind(pattern.as_deref())
        .bind(query.size)
        .bind(query.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok((rows.into_iter().map(Into::into).collect(), total))
    }

    async fn list_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
    ) -> AppResult<Vec<TaxonomyItem>> {
        let rows: Vec<TaxonomyDb> = sqlx::query_as(AssertSqlSafe(format!(
            "SELECT {COLUMNS} FROM {} WHERE user_id = ? ORDER BY name COLLATE NOCASE",
            table(kind)
        )))
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<Option<TaxonomyItem>> {
        let row: Option<TaxonomyDb> = sqlx::query_as(AssertSqlSafe(format!(
            "SELECT {COLUMNS} FROM {} WHERE id = ?",
            table(kind)
        )))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(Into::into))
    }

    async fn taxonomy_name_taken(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        name: &str,
        except_id: Option<&str>,
    ) -> AppResult<bool> {
        Ok(sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE user_id = ? AND name = ? AND id IS NOT ?)",
            table(kind)
        )))
        .bind(user_id)
        .bind(name)
        .bind(except_id)
        .fetch_one(&self.pool)
        .await?)
    }

    async fn create_taxonomy(
        &self,
        kind: TaxonomyKind,
        user_id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<TaxonomyItem> {
        let id = new_id();
        let now = now_iso();
        sqlx::query(AssertSqlSafe(format!(
            "INSERT INTO {} (id, user_id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            table(kind)
        )))
        .bind(&id)
        .bind(user_id)
        .bind(&draft.name)
        .bind(&draft.description)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.find_taxonomy(kind, &id)
            .await?
            .ok_or_else(|| AppError::internal("taxonomy item vanished after insert"))
    }

    async fn update_taxonomy(
        &self,
        kind: TaxonomyKind,
        id: &str,
        draft: &TaxonomyDraft,
    ) -> AppResult<()> {
        sqlx::query(AssertSqlSafe(format!(
            "UPDATE {} SET name = ?, description = ?, updated_at = ? WHERE id = ?",
            table(kind)
        )))
        .bind(&draft.name)
        .bind(&draft.description)
        .bind(now_iso())
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Referencing recipes keep existing thanks to `ON DELETE SET NULL`.
    async fn delete_taxonomy(&self, kind: TaxonomyKind, id: &str) -> AppResult<()> {
        sqlx::query(AssertSqlSafe(format!(
            "DELETE FROM {} WHERE id = ?",
            table(kind)
        )))
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn taxonomy_owned_by(
        &self,
        kind: TaxonomyKind,
        id: &str,
        user_id: &str,
    ) -> AppResult<bool> {
        Ok(sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE id = ? AND user_id = ?)",
            table(kind)
        )))
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?)
    }
}
