package dev.mariinkys.cocookies.domain.repository;

import dev.mariinkys.cocookies.domain.models.Category;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface CategoryRepository {
    Category save(Category category);
    Optional<Category> findById(UUID id);
    Page<Category> findAllByUserId(UUID userId, String search, Pageable pageable);
    List<Category> getSelector(UUID userId);
    void deleteById(UUID id);
    boolean existsByUserIdAndName(UUID userId, String name);
}