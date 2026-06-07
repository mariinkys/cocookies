package dev.mariinkys.cocookies.interfaces.dto.llm;

import dev.mariinkys.cocookies.domain.llm.LlmResult;

public record LlmResponse(
        LlmResult recipe
) {}

