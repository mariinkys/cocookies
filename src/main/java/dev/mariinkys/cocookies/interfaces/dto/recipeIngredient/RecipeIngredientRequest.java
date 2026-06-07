package dev.mariinkys.cocookies.interfaces.dto.recipeIngredient;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record RecipeIngredientRequest(
        @NotBlank(message = "Ingredient name is required")
        @Size(max = 150, message = "Ingredient name must not exceed 150 characters")
        String name,

        Double quantity,

        @Size(max = 50, message = "Unit must not exceed 50 characters")
        String unit,

        String notes,

        int sortOrder
) {}
