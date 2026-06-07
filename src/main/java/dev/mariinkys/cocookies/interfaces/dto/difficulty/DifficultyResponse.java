package dev.mariinkys.cocookies.interfaces.dto.difficulty;

import dev.mariinkys.cocookies.domain.models.Difficulty;

import java.time.LocalDateTime;
import java.util.UUID;

public record DifficultyResponse(
        UUID id,
        UUID userId,
        String name,
        String description,
        LocalDateTime createdAt,
        LocalDateTime updatedAt
) {
    public static DifficultyResponse from(Difficulty difficulty) {
        return new DifficultyResponse(
                difficulty.getId(),
                difficulty.getUserId(),
                difficulty.getName(),
                difficulty.getDescription(),
                difficulty.getCreatedAt(),
                difficulty.getUpdatedAt()
        );
    }
}
