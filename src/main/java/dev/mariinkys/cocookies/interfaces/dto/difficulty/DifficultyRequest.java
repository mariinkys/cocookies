package dev.mariinkys.cocookies.interfaces.dto.difficulty;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record DifficultyRequest(
        @NotBlank(message = "Name is required")
        @Size(max = 100, message = "Name must not exceed 100 characters")
        String name,

        String description
) {}