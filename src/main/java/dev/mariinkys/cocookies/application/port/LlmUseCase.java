package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.domain.llm.LlmResult;
import org.springframework.web.multipart.MultipartFile;

public interface LlmUseCase {
    LlmResult analyzeFile(MultipartFile file, String prompt);
    LlmResult analyzeUrl(String url, String prompt);
}