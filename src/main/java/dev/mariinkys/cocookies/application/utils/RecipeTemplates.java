package dev.mariinkys.cocookies.application.utils;

public enum RecipeTemplates {
    DEFAULT_ENGLISH,
    DEFAULT_SPANISH;

    public static RecipeTemplates fromString(String value) {
        if (value == null) {
            return DEFAULT_ENGLISH;
        }

        switch (value) {
            case "default-en":
                return DEFAULT_ENGLISH;
            case "default-es":
                return DEFAULT_SPANISH;
            default:
                return DEFAULT_ENGLISH;
        }
    }
}
