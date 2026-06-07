package dev.mariinkys.cocookies.domain.repository;

import dev.mariinkys.cocookies.domain.models.Difficulty;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface DifficultyRepository {
    Difficulty save(Difficulty difficulty);
    Optional<Difficulty> findById(UUID id);
    Page<Difficulty> findAllByUserId(UUID userId, String search, Pageable pageable);
    List<Difficulty> getSelector(UUID userId);
    void deleteById(UUID id);
    boolean existsByUserIdAndName(UUID userId, String name);
}