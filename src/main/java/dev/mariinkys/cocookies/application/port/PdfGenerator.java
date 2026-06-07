package dev.mariinkys.cocookies.application.port;

import java.util.Map;

public interface PdfGenerator {
    byte[] generate(String templateName, Map<String, Object> variables);
}