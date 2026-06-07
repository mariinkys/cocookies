package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.application.exception.LlmException;
import dev.mariinkys.cocookies.application.port.LlmPort;
import dev.mariinkys.cocookies.application.port.LlmUseCase;
import dev.mariinkys.cocookies.domain.llm.LlmInput;
import dev.mariinkys.cocookies.domain.llm.LlmResult;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

@Service
public class LlmService implements LlmUseCase {

    private final LlmPort llmPort;

    public LlmService(LlmPort llmPort) {
        this.llmPort = llmPort;
    }

    @Override
    public LlmResult analyzeFile(MultipartFile file, String prompt) {
        if (!llmPort.isAvailable()) {
            throw new LlmException("LLM service is not available");
        }
        try {
            String contentType = file.getContentType();
            byte[] bytes = file.getBytes();

            LlmInput input = resolveInput(contentType, bytes, file.getOriginalFilename());
            return llmPort.analyze(input, prompt);
        } catch (LlmException e) {
            throw e;
        } catch (Exception e) {
            throw new LlmException("Failed to process file: " + e.getMessage());
        }
    }

    @Override
    public LlmResult analyzeUrl(String url, String prompt) {
        if (!llmPort.isAvailable()) {
            throw new LlmException("LLM service is not available");
        }
        return llmPort.analyze(new LlmInput.Url(url), prompt);
    }

    private LlmInput resolveInput(String contentType, byte[] bytes, String filename) {
        if (contentType == null) throw new LlmException("Unknown file type");
        return switch (contentType) {
            case "application/pdf" -> new LlmInput.Pdf(bytes, filename);
            case "image/png", "image/jpeg", "image/webp", "image/gif"
                    -> new LlmInput.Image(bytes, contentType);
            default -> throw new LlmException("Unsupported file type: " + contentType);
        };
    }
}