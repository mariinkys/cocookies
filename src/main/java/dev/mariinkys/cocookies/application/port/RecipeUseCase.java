package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.application.utils.RecipeTemplates;
import dev.mariinkys.cocookies.domain.models.Recipe;
import dev.mariinkys.cocookies.domain.models.RecipeIngredient;
import dev.mariinkys.cocookies.domain.models.RecipeNutrition;
import dev.mariinkys.cocookies.domain.models.RecipeStep;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.UUID;

public interface RecipeUseCase {
    Recipe createRecipe(String title, String description, UUID categoryId, UUID difficultyId,
                        Integer prepTime, Integer cookTime, int servings, String imageUrl,
                        boolean shared, List<RecipeStep> steps,
                        List<RecipeIngredient> ingredients, RecipeNutrition nutrition, RequesterContext requester);
    Recipe getRecipeById(UUID id, RequesterContext requester);
    Page<Recipe> getAllRecipes(String search, Pageable pageable, RequesterContext requester);
    Page<Recipe> getAllSharedRecipes(String search, Pageable pageable);
    Recipe updateRecipe(UUID id, String title, String description, UUID categoryId,
                        UUID difficultyId, Integer prepTime, Integer cookTime, int servings,
                        String imageUrl, boolean shared, List<RecipeStep> steps,
                        List<RecipeIngredient> ingredients, RecipeNutrition nutrition, RequesterContext requester);
    void deleteRecipe(UUID id, RequesterContext requester);

    byte[] generateRecipePdf(UUID id, RecipeTemplates template, RequesterContext requester);
}