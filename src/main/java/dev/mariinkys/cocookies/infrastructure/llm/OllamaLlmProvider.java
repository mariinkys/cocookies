package dev.mariinkys.cocookies.infrastructure.llm;

import com.fasterxml.jackson.databind.ObjectMapper;
import dev.mariinkys.cocookies.application.exception.LlmException;
import dev.mariinkys.cocookies.application.port.LlmPort;
import dev.mariinkys.cocookies.domain.llm.LlmInput;
import dev.mariinkys.cocookies.domain.llm.LlmResult;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.ai.chat.client.ChatClient;
import org.springframework.ai.chat.messages.UserMessage;
import org.springframework.ai.content.Media;
import org.springframework.stereotype.Component;
import org.springframework.util.MimeType;


import java.util.List;

@Component
public class OllamaLlmProvider implements LlmPort {

    private static final Logger log = LoggerFactory.getLogger(OllamaLlmProvider.class);
    private static final String JSON_INSTRUCTION = """
      Extract the recipe from this content and return a JSON object with exactly this structure:
      {
        "title": "string — recipe name",
        "description": "string or null — brief description",
        "prepTime": integer or null — preparation time in minutes,
        "cookTime": integer or null — cooking time in minutes,
        "servings": integer — number of servings (default 1 if unknown),
        "imageUrl": null,
        "steps": [
          {
            "stepNumber": integer — starting from 1,
            "instructions": "string — full step description",
            "duration": integer or null — step duration in minutes
          }
        ],
        "ingredients": [
          {
            "name": "string — ingredient name",
            "quantity": number or null — numeric amount,
            "unit": "string or null — e.g. g, ml, tbsp",
            "notes": "string or null — e.g. finely chopped, room temperature",
            "sortOrder": integer — starting from 0
          }
        ],
        "nutrition": {
          "servingSizeValue": number — portion size (default 100 if unknown),
          "servingSizeUnit": "string — unit for the serving size, e.g. g, ml (default g)",
          "calories": number or null — kcal per serving size,
          "proteinG": number or null — grams of protein per serving size,
          "carbsG": number or null — grams of total carbohydrates per serving size,
          "sugarG": number or null — grams of sugars per serving size,
          "fatG": number or null — grams of total fat per serving size,
          "saturatedFatG": number or null — grams of saturated fat per serving size,
          "fiberG": number or null — grams of dietary fiber per serving size,
          "sodiumMg": number or null — milligrams of sodium per serving size
        }
      }
    
      Rules:
      - Return only the JSON object, no markdown, no explanation.
      - steps must be ordered and stepNumber must start at 1.
      - ingredients sortOrder must start at 0.
      - quantity must be a number, never a string like "2 cups" — split value and unit into separate fields.
      - If a piece of information is not present, use null.
      - For nutrition: if values are explicitly stated in the content, use them exactly.
        If not stated, estimate them using your knowledge of the ingredients and their quantities.
        Only set nutrition fields to null if you truly cannot make a reasonable estimate.
      - nutrition.servingSizeValue and nutrition.servingSizeUnit describe the reference portion
        that all nutrient values are calculated against (e.g. 100g, 1 serving).
    """;

    private final ChatClient chatClient;
    private final DocumentExtractor extractor;
    private final ObjectMapper objectMapper;

    public OllamaLlmProvider(ChatClient.Builder chatClientBuilder,
                             DocumentExtractor extractor,
                             ObjectMapper objectMapper) {
        this.chatClient = chatClientBuilder.build();
        this.extractor = extractor;
        this.objectMapper = objectMapper;
    }

    @Override
    public LlmResult analyze(LlmInput input, String prompt) {
        try {
            String response = switch (input) {
                case LlmInput.Pdf pdf -> analyzePdf(pdf, prompt);
                case LlmInput.Image image -> analyzeImage(image, prompt);
                case LlmInput.Url url -> analyzeUrl(url, prompt);
            };

            return parseResult(response);
        } catch (Exception e) {
            log.error("Failed to get LLM response: ",  e);
            throw new LlmException("Failed to get LLM response");
        }
    }

    @Override
    public boolean isAvailable() {
        try {
            chatClient.prompt().user("ping").call().content();
            return true;
        } catch (Exception e) {
            log.warn("LLM not available: {}", e.getMessage());
            return false;
        }
    }

    private String analyzePdf(LlmInput.Pdf pdf, String prompt) {
        String text = extractor.extractFromPdf(pdf.bytes());
        // Truncate to avoid context window overflow
        if (text.length() > 8000) text = text.substring(0, 8000) + "...";

        return chatClient.prompt()
                .user(JSON_INSTRUCTION + "\n\n" + prompt + "\n\nDocument content:\n" + text)
                .call()
                .content();
    }

    private String analyzeImage(LlmInput.Image image, String prompt) {
        var media = Media.builder()
                .mimeType(MimeType.valueOf(image.mimeType()))
                .data(image.bytes())
                .build();

        var message = UserMessage.builder()
                .text(JSON_INSTRUCTION + "\n\n" + prompt)
                .media(List.of(media))
                .build();

        return chatClient.prompt()
                .messages(message)
                .call()
                .content();
    }

    private String analyzeUrl(LlmInput.Url url, String prompt) {
        String text = extractor.extractFromUrl(url.url());
        if (text.length() > 8000) text = text.substring(0, 8000) + "...";

        return chatClient.prompt()
                .user(JSON_INSTRUCTION + "\n\n" + prompt + "\n\nPage content:\n" + text)
                .call()
                .content();
    }

    private LlmResult parseResult(String response) {
        try {
            String clean = response.strip()
                    .replaceAll("(?s)^```json\\s*", "")
                    .replaceAll("(?s)^```\\s*", "")
                    .replaceAll("(?s)```$", "")
                    .strip();

            return objectMapper.readValue(clean, LlmResult.class);
        } catch (Exception e) {
            log.error("Failed to parse LLM response: {}", response, e);
            throw new LlmException("LLM returned unparseable response");
        }
    }
}