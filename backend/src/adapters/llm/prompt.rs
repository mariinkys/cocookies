//! Prompt text sent to the model.

const JSON_INSTRUCTION: &str = r#"Extract the recipe from this content and return a JSON object with exactly this structure:
{
  "title": "string — recipe name",
  "description": "string or null — brief description",
  "prepTime": integer or null — preparation time in minutes,
  "cookTime": integer or null — cooking time in minutes,
  "servings": integer — number of servings (default 1 if unknown),
  "imageUrl": null,
  "steps": [
    {
      "stepNumber": integer — starting from 1,
      "instructions": "string — full step description",
      "duration": integer or null — step duration in minutes
    }
  ],
  "ingredients": [
    {
      "name": "string — ingredient name",
      "quantity": number or null — numeric amount,
      "unit": "string or null — e.g. g, ml, tbsp",
      "notes": "string or null — e.g. finely chopped, room temperature",
      "sortOrder": integer — starting from 0
    }
  ],
  "nutrition": {
    "servingSizeValue": number — portion size (default 100 if unknown),
    "servingSizeUnit": "string — unit for the serving size, e.g. g, ml (default g)",
    "calories": number or null — kcal per serving size,
    "proteinG": number or null — grams of protein per serving size,
    "carbsG": number or null — grams of total carbohydrates per serving size,
    "sugarG": number or null — grams of sugars per serving size,
    "fatG": number or null — grams of total fat per serving size,
    "saturatedFatG": number or null — grams of saturated fat per serving size,
    "fiberG": number or null — grams of dietary fiber per serving size,
    "sodiumMg": number or null — milligrams of sodium per serving size
  }
}

Rules:
- Return only the JSON object, no markdown, no explanation.
- steps must be ordered and stepNumber must start at 1.
- ingredients sortOrder must start at 0.
- quantity must be a number, never a string like "2 cups" — split value and unit into separate fields.
- If a piece of information is not present, use null.
- For nutrition: if values are explicitly stated in the content, use them exactly.
  If not stated, estimate them using your knowledge of the ingredients and their quantities.
  Only set nutrition fields to null if you truly cannot make a reasonable estimate.
- nutrition.servingSizeValue and nutrition.servingSizeUnit describe the reference portion
  that all nutrient values are calculated against (e.g. 100g, 1 serving)."#;

pub fn with_source(user_prompt: &str, label: &str, source: &str) -> String {
    format!(
        "{JSON_INSTRUCTION}\n\n{}\n\n{label}:\n{source}",
        user_prompt.trim()
    )
}

pub fn for_image(user_prompt: &str) -> String {
    format!("{JSON_INSTRUCTION}\n\n{}", user_prompt.trim())
}
