package dev.mariinkys.cocookies.domain.repository;

import dev.mariinkys.cocookies.domain.models.Recipe;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.Optional;
import java.util.UUID;

public interface RecipeRepository {
    Recipe save(Recipe recipe);
    Optional<Recipe> findById(UUID id);
    Page<Recipe> findAllByUserId(UUID userId, String search, Pageable pageable);
    Page<Recipe> findAllShared(String search, Pageable pageable);
    void deleteById(UUID id);
}