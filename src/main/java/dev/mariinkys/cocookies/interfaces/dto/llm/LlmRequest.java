package dev.mariinkys.cocookies.interfaces.dto.llm;

import jakarta.validation.constraints.NotBlank;

public record LlmRequest(
        @NotBlank(message = "Prompt is required")
        String prompt
) {}