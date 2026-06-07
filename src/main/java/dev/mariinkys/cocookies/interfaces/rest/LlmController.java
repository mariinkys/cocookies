package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.LlmUseCase;
import dev.mariinkys.cocookies.domain.llm.LlmResult;
import dev.mariinkys.cocookies.interfaces.dto.llm.LlmRequest;
import dev.mariinkys.cocookies.interfaces.dto.llm.LlmResponse;
import jakarta.validation.Valid;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

@RestController
@RequestMapping("/api/llm")
public class LlmController {

    private final LlmUseCase llmUseCase;

    public LlmController(LlmUseCase llmUseCase) {
        this.llmUseCase = llmUseCase;
    }

    @PostMapping(value = "/analyze/file", consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<LlmResponse> analyzeFile(
            @RequestPart("file") MultipartFile file,
            @RequestPart("data") @Valid LlmRequest request) {
        LlmResult result = llmUseCase.analyzeFile(file, request.prompt());
        return ResponseEntity.ok(new LlmResponse(result));
    }

    @PostMapping("/analyze/url")
    public ResponseEntity<LlmResponse> analyzeUrl(
            @RequestParam String url,
            @Valid @RequestBody LlmRequest request) {
        LlmResult result = llmUseCase.analyzeUrl(url, request.prompt());
        return ResponseEntity.ok(new LlmResponse(result));
    }
}