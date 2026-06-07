package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.Category;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.CategoryJpaEntity;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class CategoryMapper {

    public Category toDomain(CategoryJpaEntity entity) {
        return new Category(
                UUID.fromString(entity.getId()),
                UUID.fromString(entity.getUserId()),
                entity.getName(),
                entity.getDescription(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    public CategoryJpaEntity toEntity(Category category) {
        return new CategoryJpaEntity(
                category.getId().toString(),
                category.getUserId().toString(),
                category.getName(),
                category.getDescription(),
                category.getCreatedAt(),
                category.getUpdatedAt()
        );
    }
}