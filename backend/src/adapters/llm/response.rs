//! Lenient parsing of the model's JSON answer into the domain's [`ExtractedRecipe`].
//!
//! Small local models are sloppy with types (`"servings": "4"`, `"quantity": "1/2"`), so every
//! field is parsed leniently; business defaults are applied later by the domain.

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::entities::{
    imported_recipe::{ExtractedIngredient, ExtractedRecipe, ExtractedStep},
    recipe::{NutrientValues, NutritionDraft},
};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct LlmRecipe {
    #[serde(deserialize_with = "lenient_string")]
    title: String,
    #[serde(deserialize_with = "lenient_opt_string")]
    description: Option<String>,
    #[serde(deserialize_with = "lenient_int")]
    prep_time: Option<i64>,
    #[serde(deserialize_with = "lenient_int")]
    cook_time: Option<i64>,
    #[serde(deserialize_with = "lenient_int")]
    servings: Option<i64>,
    steps: Vec<LlmStep>,
    ingredients: Vec<LlmIngredient>,
    nutrition: Option<LlmNutrition>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct LlmStep {
    #[serde(deserialize_with = "lenient_int")]
    step_number: Option<i64>,
    #[serde(deserialize_with = "lenient_string")]
    instructions: String,
    #[serde(deserialize_with = "lenient_int")]
    duration: Option<i64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct LlmIngredient {
    #[serde(deserialize_with = "lenient_string")]
    name: String,
    #[serde(deserialize_with = "lenient_float")]
    quantity: Option<f64>,
    #[serde(deserialize_with = "lenient_opt_string")]
    unit: Option<String>,
    #[serde(deserialize_with = "lenient_opt_string")]
    notes: Option<String>,
    #[serde(deserialize_with = "lenient_int")]
    sort_order: Option<i64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct LlmNutrition {
    #[serde(deserialize_with = "lenient_float")]
    serving_size_value: Option<f64>,
    #[serde(deserialize_with = "lenient_opt_string")]
    serving_size_unit: Option<String>,
    #[serde(deserialize_with = "lenient_float")]
    calories: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    protein_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    carbs_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    sugar_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    fat_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    saturated_fat_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    fiber_g: Option<f64>,
    #[serde(deserialize_with = "lenient_float")]
    sodium_mg: Option<f64>,
}

impl From<LlmRecipe> for ExtractedRecipe {
    fn from(r: LlmRecipe) -> Self {
        ExtractedRecipe {
            title: r.title,
            description: r.description,
            prep_time: r.prep_time,
            cook_time: r.cook_time,
            servings: r.servings,
            steps: r
                .steps
                .into_iter()
                .map(|s| ExtractedStep {
                    step_number: s.step_number,
                    instructions: s.instructions,
                    duration: s.duration,
                })
                .collect(),
            ingredients: r
                .ingredients
                .into_iter()
                .map(|i| ExtractedIngredient {
                    name: i.name,
                    quantity: i.quantity,
                    unit: i.unit,
                    notes: i.notes,
                })
                .collect(),
            nutrition: r.nutrition.map(|n| NutritionDraft {
                serving_size_value: n.serving_size_value,
                serving_size_unit: n.serving_size_unit.unwrap_or_default(),
                values: NutrientValues {
                    calories: n.calories,
                    protein_g: n.protein_g,
                    carbs_g: n.carbs_g,
                    sugar_g: n.sugar_g,
                    fat_g: n.fat_g,
                    saturated_fat_g: n.saturated_fat_g,
                    fiber_g: n.fiber_g,
                    sodium_mg: n.sodium_mg,
                },
            }),
        }
    }
}

/// Parses the model output, tolerating Markdown code fences and leading chatter.
pub fn parse_model_output(raw: &str) -> Option<ExtractedRecipe> {
    let trimmed = raw.trim();
    let start = trimmed.find('{')?;
    let end = trimmed.rfind('}')?;
    serde_json::from_str::<LlmRecipe>(trimmed.get(start..=end)?)
        .ok()
        .map(Into::into)
}

fn number(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => parse_number(s),
        _ => None,
    }
}

/// "2", "2.5", "2,5", "1/2", "1 1/2", "3 cups" (leading number) → f64.
fn parse_number(s: &str) -> Option<f64> {
    let s = s.trim().replace(',', ".");
    let mut total = 0.0;
    let mut any = false;
    for part in s.split_whitespace() {
        let value = match part.split_once('/') {
            Some((a, b)) => match (a.parse::<f64>(), b.parse::<f64>()) {
                (Ok(a), Ok(b)) if b != 0.0 => a / b,
                _ => break,
            },
            None => match part.parse::<f64>() {
                Ok(v) => v,
                Err(_) => break,
            },
        };
        total += value;
        any = true;
    }
    (any && total.is_finite()).then_some(total)
}

fn lenient_float<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    Ok(number(&Value::deserialize(d)?))
}

fn lenient_int<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    Ok(number(&Value::deserialize(d)?).map(|f| f.round() as i64))
}

fn lenient_opt_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    })
}

fn lenient_string<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    Ok(lenient_opt_string(d)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sloppy_output() {
        let raw = r#"Sure! ```json
        {"title": " Pancakes ", "servings": "4", "prepTime": 10.4,
         "steps": [{"stepNumber": 2, "instructions": "Cook"}, {"stepNumber": 1, "instructions": "Mix"}],
         "ingredients": [{"name": "Flour", "quantity": "1 1/2", "unit": "cups"}, {"name": "Milk", "quantity": "2,5"}, {"name": 7}],
         "nutrition": {"calories": "200", "servingSizeUnit": null}, "imageUrl": "http://made.up/img.png"}
        ```"#;
        let r = parse_model_output(raw).unwrap();
        assert_eq!(r.title, "Pancakes");
        assert_eq!((r.servings, r.prep_time), (Some(4), Some(10)));
        assert_eq!(r.steps[0].step_number, Some(2));
        assert_eq!(r.ingredients[0].quantity, Some(1.5));
        assert_eq!(r.ingredients[1].quantity, Some(2.5));
        assert_eq!(r.ingredients[2].name, "7");
        assert_eq!(r.nutrition.unwrap().values.calories, Some(200.0));
        assert!(parse_model_output("no json here").is_none());
    }
}
