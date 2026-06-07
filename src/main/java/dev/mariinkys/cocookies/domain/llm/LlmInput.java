package dev.mariinkys.cocookies.domain.llm;

public sealed interface LlmInput permits LlmInput.Pdf, LlmInput.Image, LlmInput.Url {
    record Pdf(byte[] bytes, String filename) implements LlmInput {}
    record Image(byte[] bytes, String mimeType) implements LlmInput {}
    record Url(String url) implements LlmInput {}
}