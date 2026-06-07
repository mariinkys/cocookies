package dev.mariinkys.cocookies.interfaces.dto.recipeStep;

import jakarta.validation.constraints.Min;
import jakarta.validation.constraints.NotBlank;

public record RecipeStepRequest(
        @Min(value = 1, message = "Step number must be at least 1")
        int stepNumber,

        @NotBlank(message = "Instructions are required")
        String instructions,

        Integer duration
) {}
