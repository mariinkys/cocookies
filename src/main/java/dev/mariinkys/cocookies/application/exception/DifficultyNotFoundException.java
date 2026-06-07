package dev.mariinkys.cocookies.application.exception;

import java.util.UUID;

public class DifficultyNotFoundException extends RuntimeException {
    public DifficultyNotFoundException(UUID id) {
        super("Difficulty not found with id: " + id);
    }
}
