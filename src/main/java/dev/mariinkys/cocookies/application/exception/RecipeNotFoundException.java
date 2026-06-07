package dev.mariinkys.cocookies.application.exception;

import java.util.UUID;

public class RecipeNotFoundException extends RuntimeException {
  public RecipeNotFoundException(UUID id) {
    super("Recipe not found with id: " + id);
  }
}
