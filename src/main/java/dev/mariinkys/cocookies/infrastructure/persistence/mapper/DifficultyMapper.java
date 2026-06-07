package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.Difficulty;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.DifficultyJpaEntity;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class DifficultyMapper {

    public Difficulty toDomain(DifficultyJpaEntity entity) {
        return new Difficulty(
                UUID.fromString(entity.getId()),
                UUID.fromString(entity.getUserId()),
                entity.getName(),
                entity.getDescription(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    public DifficultyJpaEntity toEntity(Difficulty difficulty) {
        return new DifficultyJpaEntity(
                difficulty.getId().toString(),
                difficulty.getUserId().toString(),
                difficulty.getName(),
                difficulty.getDescription(),
                difficulty.getCreatedAt(),
                difficulty.getUpdatedAt()
        );
    }
}