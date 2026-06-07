package dev.mariinkys.cocookies.infrastructure.pdf;

import com.openhtmltopdf.pdfboxout.PdfRendererBuilder;
import dev.mariinkys.cocookies.application.port.PdfGenerator;
import org.springframework.stereotype.Component;
import org.thymeleaf.TemplateEngine;
import org.thymeleaf.context.Context;

import java.io.ByteArrayOutputStream;
import java.util.Map;

@Component
public class ThymeleafPdfGenerator implements PdfGenerator {

    private final TemplateEngine templateEngine;

    public ThymeleafPdfGenerator(TemplateEngine templateEngine) {
        this.templateEngine = templateEngine;
    }

    @Override
    public byte[] generate(String templateName, Map<String, Object> variables) {
        // render HTML with Thymeleaf
        var context = new Context();
        context.setVariables(variables);
        String html = templateEngine.process(templateName, context);

        // convert HTML to PDF
        try (var out = new ByteArrayOutputStream()) {
            new PdfRendererBuilder()
                    .withHtmlContent(html, null)
                    .toStream(out)
                    .run();
            return out.toByteArray();
        } catch (Exception e) {
            throw new RuntimeException("Failed to generate PDF: " + e.getMessage(), e);
        }
    }
}