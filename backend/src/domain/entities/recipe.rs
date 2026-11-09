use std::collections::HashSet;

use super::taxonomy::TaxonomyItem;
use crate::domain::validation::{ValidationError, Violations, max_chars, normalize_opt, not_blank};

/// A full recipe with its children.
#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub user_id: String,
    pub category: Option<TaxonomyItem>,
    pub difficulty: Option<TaxonomyItem>,
    pub title: String,
    pub description: Option<String>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub servings: i64,
    pub image_url: Option<String>,
    pub shared: bool,
    pub steps: Vec<Step>,
    pub ingredients: Vec<Ingredient>,
    pub nutrition: Option<Nutrition>,
    pub created_at: String,
    pub updated_at: String,
}

/// The lightweight "card" shown in listings: no children, and only the names of the
/// category/difficulty. Listings are the hottest reads, so they stay small.
#[derive(Debug, Clone)]
pub struct RecipeSummary {
    pub id: String,
    pub user_id: String,
    pub category: Option<NamedRef>,
    pub difficulty: Option<NamedRef>,
    pub title: String,
    pub description: Option<String>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub servings: i64,
    pub image_url: Option<String>,
    pub shared: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct NamedRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub id: i64,
    pub step_number: i64,
    pub instructions: String,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub notes: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone)]
pub struct Nutrition {
    pub id: i64,
    pub serving_size_value: f64,
    pub serving_size_unit: String,
    pub values: NutrientValues,
}

/// Nutrients per serving size; all optional.
#[derive(Debug, Clone, Default)]
pub struct NutrientValues {
    pub calories: Option<f64>,
    pub protein_g: Option<f64>,
    pub carbs_g: Option<f64>,
    pub sugar_g: Option<f64>,
    pub fat_g: Option<f64>,
    pub saturated_fat_g: Option<f64>,
    pub fiber_g: Option<f64>,
    pub sodium_mg: Option<f64>,
}

// ---------------------------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------------------------

/// A recipe as submitted by a user (create / update), or as extracted by the AI import.
#[derive(Debug, Clone, Default)]
pub struct RecipeDraft {
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub difficulty_id: Option<String>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub servings: i64,
    pub image_url: Option<String>,
    pub shared: bool,
    pub steps: Vec<StepDraft>,
    pub ingredients: Vec<IngredientDraft>,
    pub nutrition: Option<NutritionDraft>,
}

#[derive(Debug, Clone, Default)]
pub struct StepDraft {
    pub step_number: i64,
    pub instructions: String,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct IngredientDraft {
    pub name: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub notes: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Default)]
pub struct NutritionDraft {
    pub serving_size_value: Option<f64>,
    pub serving_size_unit: String,
    pub values: NutrientValues,
}

impl RecipeDraft {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        v.check(not_blank(&self.title), "Title is required")
            .check(
                max_chars(self.title.trim(), 255),
                "Title must not exceed 255 characters",
            )
            .check(self.servings >= 1, "Servings must be at least 1")
            .check(
                self.prep_time.is_none_or(|t| t >= 0),
                "Prep time must not be negative",
            )
            .check(
                self.cook_time.is_none_or(|t| t >= 0),
                "Cook time must not be negative",
            )
            .check(
                self.image_url.as_deref().is_none_or(|u| max_chars(u, 4096)),
                "Image URL must not exceed 4096 characters",
            )
            .check(!self.steps.is_empty(), "At least one step is required")
            .check(
                !self.ingredients.is_empty(),
                "At least one ingredient is required",
            );

        let mut seen = HashSet::with_capacity(self.steps.len());
        for s in &self.steps {
            v.check(s.step_number >= 1, "Step number must be at least 1")
                .check(not_blank(&s.instructions), "Instructions are required")
                .check(
                    s.duration.is_none_or(|d| d >= 0),
                    "Step duration must not be negative",
                )
                .check(
                    seen.insert(s.step_number),
                    format!("Step number {} is duplicated", s.step_number),
                );
        }
        for i in &self.ingredients {
            v.check(not_blank(&i.name), "Ingredient name is required")
                .check(
                    max_chars(i.name.trim(), 150),
                    "Ingredient name must not exceed 150 characters",
                )
                .check(
                    i.unit.as_deref().is_none_or(|u| max_chars(u.trim(), 50)),
                    "Unit must not exceed 50 characters",
                );
        }
        if let Some(n) = &self.nutrition {
            v.check(
                n.serving_size_value.is_some(),
                "Serving size value is required",
            )
            .check(
                n.serving_size_value.is_none_or(|x| x > 0.0),
                "Serving size value must be positive",
            )
            .check(
                not_blank(&n.serving_size_unit),
                "Serving size unit is required",
            );
        }
        v.finish()
    }

    /// Trims text and turns empty optional strings into `None`, ready to be stored.
    pub fn normalized(mut self) -> Self {
        self.title = self.title.trim().to_owned();
        self.description = normalize_opt(self.description);
        self.image_url = normalize_opt(self.image_url);
        for s in &mut self.steps {
            s.instructions = s.instructions.trim().to_owned();
        }
        for i in &mut self.ingredients {
            i.name = i.name.trim().to_owned();
            i.unit = normalize_opt(i.unit.take());
            i.notes = normalize_opt(i.notes.take());
        }
        if let Some(n) = &mut self.nutrition {
            n.serving_size_unit = n.serving_size_unit.trim().to_owned();
        }
        self
    }
}

// ---------------------------------------------------------------------------------------------
// Listing & rendering
// ---------------------------------------------------------------------------------------------

/// Which recipes a listing covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeScope {
    /// Every recipe owned by this user (shared or not).
    OwnedBy(String),
    /// Every shared recipe, whoever owns it.
    Shared,
}

/// Fields recipe listings can be sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecipeSort {
    #[default]
    Title,
    CreatedAt,
    UpdatedAt,
    PrepTime,
    CookTime,
    Servings,
}

/// Printable layouts of a recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfTemplate {
    DefaultEnglish,
    DefaultSpanish,
}

impl PdfTemplate {
    /// `default-en` / `default-es`; anything else falls back to English.
    pub fn parse(value: &str) -> Self {
        match value {
            "default-es" => Self::DefaultSpanish,
            _ => Self::DefaultEnglish,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> RecipeDraft {
        RecipeDraft {
            title: "  Tortilla ".into(),
            description: Some("   ".into()),
            servings: 2,
            steps: vec![StepDraft {
                step_number: 1,
                instructions: " Mix ".into(),
                duration: None,
            }],
            ingredients: vec![IngredientDraft {
                name: " Eggs ".into(),
                unit: Some(String::new()),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn validates_and_normalizes() {
        assert!(draft().validate().is_ok());
        let d = draft().normalized();
        assert_eq!(d.title, "Tortilla");
        assert_eq!(d.description, None);
        assert_eq!(d.steps[0].instructions, "Mix");
        assert_eq!(d.ingredients[0].unit, None);

        let mut bad = draft();
        bad.servings = 0;
        bad.steps.push(bad.steps[0].clone());
        let err = bad.validate().unwrap_err().0;
        assert!(err.contains("Servings must be at least 1"));
        assert!(err.contains("Step number 1 is duplicated"));
    }

    #[test]
    fn template_parsing() {
        assert_eq!(
            PdfTemplate::parse("default-es"),
            PdfTemplate::DefaultSpanish
        );
        assert_eq!(PdfTemplate::parse("anything"), PdfTemplate::DefaultEnglish);
    }
}
