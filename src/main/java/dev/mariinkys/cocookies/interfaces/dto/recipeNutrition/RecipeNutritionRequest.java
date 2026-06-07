package dev.mariinkys.cocookies.interfaces.dto.recipeNutrition;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Positive;

public record RecipeNutritionRequest(
        @NotNull(message = "Serving size value is required")
        @Positive(message = "Serving size value must be positive")
        Double servingSizeValue,

        @NotBlank(message = "Serving size unit is required")
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