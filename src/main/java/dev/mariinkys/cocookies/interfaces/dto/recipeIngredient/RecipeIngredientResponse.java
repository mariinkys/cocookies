package dev.mariinkys.cocookies.interfaces.dto.recipeIngredient;

import dev.mariinkys.cocookies.domain.models.RecipeIngredient;

public record RecipeIngredientResponse(
        Long id,
        String name,
        Double quantity,
        String unit,
        String notes,
        int sortOrder
) {
    public static RecipeIngredientResponse from(RecipeIngredient ingredient) {
        return new RecipeIngredientResponse(
                ingredient.getId(),
                ingredient.getName(),
                ingredient.getQuantity(),
                ingredient.getUnit(),
                ingredient.getNotes(),
                ingredient.getSortOrder()
        );
    }
}
