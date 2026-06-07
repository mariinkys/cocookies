package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.domain.llm.LlmInput;
import dev.mariinkys.cocookies.domain.llm.LlmResult;

public interface LlmPort {
    LlmResult analyze(LlmInput input, String prompt);
    boolean isAvailable();
}