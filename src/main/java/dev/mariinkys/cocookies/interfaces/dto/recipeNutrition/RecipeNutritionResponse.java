package dev.mariinkys.cocookies.interfaces.dto.recipeNutrition;

import dev.mariinkys.cocookies.domain.models.RecipeNutrition;

public record RecipeNutritionResponse(
        Long id,
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
) {
    public static RecipeNutritionResponse from(RecipeNutrition nutrition) {
        return new RecipeNutritionResponse(
                nutrition.getId(),
                nutrition.getServingSizeValue(),
                nutrition.getServingSizeUnit(),
                nutrition.getCalories(),
                nutrition.getProteinG(),
                nutrition.getCarbsG(),
                nutrition.getSugarG(),
                nutrition.getFatG(),
                nutrition.getSaturatedFatG(),
                nutrition.getFiberG(),
                nutrition.getSodiumMg()
        );
    }
}