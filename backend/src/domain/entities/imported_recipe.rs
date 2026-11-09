//! A recipe as extracted by the AI import, before it is turned into a [`RecipeDraft`] the user
//! can review. Models are sloppy, so every field is optional here and
//! [`ExtractedRecipe::into_draft`] applies the business defaults.

use super::recipe::{IngredientDraft, NutritionDraft, RecipeDraft, StepDraft};

#[derive(Debug, Clone, Default)]
pub struct ExtractedRecipe {
    pub title: String,
    pub description: Option<String>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub servings: Option<i64>,
    pub steps: Vec<ExtractedStep>,
    pub ingredients: Vec<ExtractedIngredient>,
    pub nutrition: Option<NutritionDraft>,
}

#[derive(Debug, Clone, Default)]
pub struct ExtractedStep {
    pub step_number: Option<i64>,
    pub instructions: String,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ExtractedIngredient {
    pub name: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub notes: Option<String>,
}

impl ExtractedRecipe {
    /// Makes the extraction safe to drop straight into the recipe form: trimmed strings, steps
    /// numbered from 1 in the model's order, ingredients ordered from 0, sane defaults. Images
    /// are never taken from the model (they hallucinate URLs).
    pub fn into_draft(self) -> RecipeDraft {
        let mut steps: Vec<ExtractedStep> = self
            .steps
            .into_iter()
            .filter(|s| !s.instructions.trim().is_empty())
            .collect();
        steps.sort_by_key(|s| s.step_number.unwrap_or(i64::MAX));

        let nutrition = self.nutrition.map(|mut n| {
            n.serving_size_value = Some(n.serving_size_value.filter(|v| *v > 0.0).unwrap_or(100.0));
            if n.serving_size_unit.trim().is_empty() {
                n.serving_size_unit = "g".into();
            }
            n
        });

        RecipeDraft {
            title: self.title.trim().to_owned(),
            description: self.description,
            category_id: None,
            difficulty_id: None,
            prep_time: self.prep_time.filter(|t| *t >= 0),
            cook_time: self.cook_time.filter(|t| *t >= 0),
            servings: self.servings.filter(|s| *s >= 1).unwrap_or(1),
            image_url: None,
            shared: false,
            steps: steps
                .into_iter()
                .enumerate()
                .map(|(i, s)| StepDraft {
                    step_number: i as i64 + 1,
                    instructions: s.instructions.trim().to_owned(),
                    duration: s.duration.filter(|d| *d >= 0),
                })
                .collect(),
            ingredients: self
                .ingredients
                .into_iter()
                .filter(|i| !i.name.trim().is_empty())
                .enumerate()
                .map(|(n, i)| IngredientDraft {
                    name: i.name.trim().to_owned(),
                    quantity: i.quantity,
                    unit: i.unit,
                    notes: i.notes,
                    sort_order: n as i64,
                })
                .collect(),
            nutrition,
        }
        .normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_defaults_and_ordering() {
        let extracted = ExtractedRecipe {
            title: " Pancakes ".into(),
            servings: Some(0),
            prep_time: Some(-1),
            steps: vec![
                ExtractedStep {
                    step_number: Some(2),
                    instructions: "Cook".into(),
                    duration: None,
                },
                ExtractedStep {
                    step_number: Some(1),
                    instructions: "Mix".into(),
                    duration: Some(-5),
                },
                ExtractedStep {
                    step_number: None,
                    instructions: "  ".into(),
                    duration: None,
                },
            ],
            ingredients: vec![
                ExtractedIngredient {
                    name: " ".into(),
                    ..Default::default()
                },
                ExtractedIngredient {
                    name: "Milk".into(),
                    ..Default::default()
                },
            ],
            nutrition: Some(NutritionDraft::default()),
            ..Default::default()
        };
        let d = extracted.into_draft();
        assert_eq!(d.title, "Pancakes");
        assert_eq!(d.servings, 1);
        assert_eq!(d.prep_time, None);
        assert_eq!(d.steps.len(), 2);
        assert_eq!(
            (d.steps[0].step_number, d.steps[0].instructions.as_str()),
            (1, "Mix")
        );
        assert_eq!(d.steps[0].duration, None);
        assert_eq!((d.ingredients.len(), d.ingredients[0].sort_order), (1, 0));
        let n = d.nutrition.unwrap();
        assert_eq!(
            (n.serving_size_value, n.serving_size_unit.as_str()),
            (Some(100.0), "g")
        );
    }
}
