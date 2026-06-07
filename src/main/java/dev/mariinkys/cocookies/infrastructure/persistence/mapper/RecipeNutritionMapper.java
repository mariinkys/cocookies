package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.RecipeNutrition;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeNutritionJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class RecipeNutritionMapper {

    public RecipeNutrition toDomain(RecipeNutritionJpaEntity entity) {
        return new RecipeNutrition(
                entity.getId(),
                entity.getServingSizeValue(),
                entity.getServingSizeUnit(),
                entity.getCalories(),
                entity.getProteinG(),
                entity.getCarbsG(),
                entity.getSugarG(),
                entity.getFatG(),
                entity.getSaturatedFatG(),
                entity.getFiberG(),
                entity.getSodiumMg()
        );
    }

    public RecipeNutritionJpaEntity toEntity(RecipeNutrition nutrition, RecipeJpaEntity recipeEntity) {
        return new RecipeNutritionJpaEntity(
                recipeEntity,
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