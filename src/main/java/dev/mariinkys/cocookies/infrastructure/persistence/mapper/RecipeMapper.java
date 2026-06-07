package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.Recipe;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.CategoryJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.DifficultyJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import org.springframework.stereotype.Component;

import java.util.UUID;
import java.util.stream.Collectors;

@Component
public class RecipeMapper {

    private final CategoryMapper categoryMapper;
    private final DifficultyMapper difficultyMapper;
    private final RecipeStepMapper stepMapper;
    private final RecipeIngredientMapper ingredientMapper;
    private final RecipeNutritionMapper nutritionMapper;

    public RecipeMapper(CategoryMapper categoryMapper, DifficultyMapper difficultyMapper,
                        RecipeStepMapper stepMapper, RecipeIngredientMapper ingredientMapper,
                        RecipeNutritionMapper nutritionMapper) {
        this.categoryMapper = categoryMapper;
        this.difficultyMapper = difficultyMapper;
        this.stepMapper = stepMapper;
        this.ingredientMapper = ingredientMapper;
        this.nutritionMapper = nutritionMapper;
    }

    public Recipe toDomain(RecipeJpaEntity entity) {
        return new Recipe(
                UUID.fromString(entity.getId()),
                UUID.fromString(entity.getUserId()),
                entity.getCategory() != null ? categoryMapper.toDomain(entity.getCategory()) : null,
                entity.getDifficulty() != null ? difficultyMapper.toDomain(entity.getDifficulty()) : null,
                entity.getTitle(),
                entity.getDescription(),
                entity.getPrepTime(),
                entity.getCookTime(),
                entity.getServings(),
                entity.getImageUrl(),
                entity.isShared(),
                entity.getSteps().stream().map(stepMapper::toDomain).toList(),
                entity.getIngredients().stream().map(ingredientMapper::toDomain).toList(),
                entity.getNutrition() != null ? nutritionMapper.toDomain(entity.getNutrition()) : null,
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    public RecipeJpaEntity toEntity(Recipe recipe,
                                    CategoryJpaEntity categoryEntity,
                                    DifficultyJpaEntity difficultyEntity) {
        RecipeJpaEntity entity = new RecipeJpaEntity(
                recipe.getId().toString(),
                recipe.getUserId().toString(),
                categoryEntity,
                difficultyEntity,
                recipe.getTitle(),
                recipe.getDescription(),
                recipe.getPrepTime(),
                recipe.getCookTime(),
                recipe.getServings(),
                recipe.getImageUrl(),
                recipe.isShared(),
                recipe.getCreatedAt(),
                recipe.getUpdatedAt()
        );

        entity.setSteps(
                recipe.getSteps().stream()
                        .map(s -> stepMapper.toEntity(s, entity))
                        .collect(Collectors.toSet())
        );
        entity.setIngredients(
                recipe.getIngredients().stream()
                        .map(i -> ingredientMapper.toEntity(i, entity))
                        .collect(Collectors.toSet())
        );
        if (recipe.getNutrition() != null) {
            entity.setNutrition(nutritionMapper.toEntity(recipe.getNutrition(), entity));
        }

        return entity;
    }
}