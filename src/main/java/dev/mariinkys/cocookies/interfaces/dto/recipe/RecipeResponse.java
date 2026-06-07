package dev.mariinkys.cocookies.interfaces.dto.recipe;

import dev.mariinkys.cocookies.domain.models.Recipe;
import dev.mariinkys.cocookies.interfaces.dto.category.CategoryResponse;
import dev.mariinkys.cocookies.interfaces.dto.difficulty.DifficultyResponse;
import dev.mariinkys.cocookies.interfaces.dto.recipeIngredient.RecipeIngredientResponse;
import dev.mariinkys.cocookies.interfaces.dto.recipeNutrition.RecipeNutritionResponse;
import dev.mariinkys.cocookies.interfaces.dto.recipeStep.RecipeStepResponse;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public record RecipeResponse(
        UUID id,
        UUID userId,
        CategoryResponse category,
        DifficultyResponse difficulty,
        String title,
        String description,
        Integer prepTime,
        Integer cookTime,
        int servings,
        String imageUrl,
        boolean shared,
        List<RecipeStepResponse> steps,
        List<RecipeIngredientResponse> ingredients,
        RecipeNutritionResponse nutrition,
        LocalDateTime createdAt,
        LocalDateTime updatedAt
) {
    public static RecipeResponse from(Recipe recipe) {
        return new RecipeResponse(
                recipe.getId(),
                recipe.getUserId(),
                recipe.getCategory() != null ? CategoryResponse.from(recipe.getCategory()) : null,
                recipe.getDifficulty() != null ? DifficultyResponse.from(recipe.getDifficulty()) : null,
                recipe.getTitle(),
                recipe.getDescription(),
                recipe.getPrepTime(),
                recipe.getCookTime(),
                recipe.getServings(),
                recipe.getImageUrl(),
                recipe.isShared(),
                recipe.getSteps().stream().map(RecipeStepResponse::from).toList(),
                recipe.getIngredients().stream().map(RecipeIngredientResponse::from).toList(),
                recipe.getNutrition() != null ? RecipeNutritionResponse.from(recipe.getNutrition()) : null,
                recipe.getCreatedAt(),
                recipe.getUpdatedAt()
        );
    }
}
