package dev.mariinkys.cocookies.domain.llm;

import java.util.List;

public record LlmResult(
        String title,
        String description,
        Integer prepTime,
        Integer cookTime,
        Integer servings,
        String imageUrl,
        List<LlmStepResult> steps,
        List<LlmIngredientResult> ingredients,
        LlmNutritionResult nutrition
) {
    public record LlmStepResult(
            int stepNumber,
            String instructions,
            Integer duration
    ) {}

    public record LlmIngredientResult(
            String name,
            Double quantity,
            String unit,
            String notes,
            int sortOrder
    ) {}

    public record LlmNutritionResult(
            Double servingSizeValue,
            String servingSizeUnit,
            Double calories,
            Double proteinG,
            Double carbsG,
            Double sugarG,
            Double fatG,
            Double saturatedFatG,
            Double fiberG,
            Double sodiumMg
    ) {}
}