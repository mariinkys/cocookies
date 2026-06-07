package dev.mariinkys.cocookies.interfaces.dto.recipeStep;

import dev.mariinkys.cocookies.domain.models.RecipeStep;

public record RecipeStepResponse(
        Long id,
        int stepNumber,
        String instructions,
        Integer duration
) {
    public static RecipeStepResponse from(RecipeStep step) {
        return new RecipeStepResponse(
                step.getId(),
                step.getStepNumber(),
                step.getInstructions(),
                step.getDuration()
        );
    }
}
