package dev.mariinkys.cocookies.interfaces.dto.recipe;

import dev.mariinkys.cocookies.interfaces.dto.recipeIngredient.RecipeIngredientRequest;
import dev.mariinkys.cocookies.interfaces.dto.recipeNutrition.RecipeNutritionRequest;
import dev.mariinkys.cocookies.interfaces.dto.recipeStep.RecipeStepRequest;
import jakarta.validation.constraints.Min;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.Size;

import java.util.List;
import java.util.UUID;

public record RecipeRequest(
        @NotBlank(message = "Title is required")
        @Size(max = 255, message = "Title must not exceed 255 characters")
        String title,

        String description,

        UUID categoryId,
        UUID difficultyId,

        Integer prepTime,
        Integer cookTime,

        @Min(value = 1, message = "Servings must be at least 1")
        int servings,

        String imageUrl,
        boolean shared,

        @NotEmpty(message = "At least one step is required")
        List<RecipeStepRequest> steps,

        @NotEmpty(message = "At least one ingredient is required")
        List<RecipeIngredientRequest> ingredients,

        RecipeNutritionRequest nutrition
) {}
