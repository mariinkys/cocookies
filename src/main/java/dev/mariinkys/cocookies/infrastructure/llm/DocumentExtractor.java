package dev.mariinkys.cocookies.infrastructure.llm;

import org.apache.pdfbox.Loader;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;
import org.springframework.stereotype.Component;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

@Component
public class DocumentExtractor {

    private final HttpClient httpClient = HttpClient.newHttpClient();

    public String extractFromPdf(byte[] bytes) {
        try (PDDocument doc = Loader.loadPDF(bytes)) {
            return new PDFTextStripper().getText(doc);
        } catch (IOException e) {
            throw new RuntimeException("Failed to extract PDF text: " + e.getMessage());
        }
    }

    public String extractFromUrl(String url) {
        try {
            var request = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .header("User-Agent", "Cocookies/1.0")
                    .GET()
                    .build();
            var response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());
            // Strip HTML tags for cleaner LLM input
            return response.body().replaceAll("<[^>]+>", " ").replaceAll("\\s+", " ").trim();
        } catch (Exception e) {
            throw new RuntimeException("Failed to fetch URL: " + e.getMessage());
        }
    }
}