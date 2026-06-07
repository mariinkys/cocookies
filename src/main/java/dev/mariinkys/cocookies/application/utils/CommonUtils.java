package dev.mariinkys.cocookies.application.utils;

import java.time.LocalDate;
import java.time.format.DateTimeParseException;

public class CommonUtils {
    public static String nullToEmpty(String value) {
        return value == null ? "" : value;
    }

    public static LocalDate parseDate(String date) {
        if (date == null || date.isBlank()) return null;
        try {
            return LocalDate.parse(date);
        } catch (DateTimeParseException e) {
            throw new IllegalArgumentException("Invalid date format. Expected yyyy-MM-dd");
        }
    }
}