package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.RecipeStep;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeStepJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class RecipeStepMapper {

    public RecipeStep toDomain(RecipeStepJpaEntity entity) {
        return new RecipeStep(
                entity.getId(),
                entity.getStepNumber(),
                entity.getInstructions(),
                entity.getDuration()
        );
    }

    public RecipeStepJpaEntity toEntity(RecipeStep step, RecipeJpaEntity recipeEntity) {
        return new RecipeStepJpaEntity(
                recipeEntity,
                step.getStepNumber(),
                step.getInstructions(),
                step.getDuration()
        );
    }
}