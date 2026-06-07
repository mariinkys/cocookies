package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.RecipeIngredient;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeIngredientJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class RecipeIngredientMapper {

    public RecipeIngredient toDomain(RecipeIngredientJpaEntity entity) {
        return new RecipeIngredient(
                entity.getId(),
                entity.getName(),
                entity.getQuantity(),
                entity.getUnit(),
                entity.getNotes(),
                entity.getSortOrder()
        );
    }

    public RecipeIngredientJpaEntity toEntity(RecipeIngredient ingredient, RecipeJpaEntity recipeEntity) {
        return new RecipeIngredientJpaEntity(
                recipeEntity,
                ingredient.getName(),
                ingredient.getQuantity(),
                ingredient.getUnit(),
                ingredient.getNotes(),
                ingredient.getSortOrder()
        );
    }
}