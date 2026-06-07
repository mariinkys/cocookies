package dev.mariinkys.cocookies.domain.models;

import java.time.LocalDateTime;
import java.util.UUID;

public class Category {

    private final UUID id;
    private final UUID userId;
    private final String name;
    private final String description;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    // For creating a new category
    public Category(UUID userId, String name, String description) {
        this.id = UUID.randomUUID();
        this.userId = userId;
        this.name = name;
        this.description = description;
    }

    // For reconstructing from DB
    public Category(UUID id, UUID userId, String name, String description,
                    LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.userId = userId;
        this.name = name;
        this.description = description;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public Category withUpdatedDetails(String name, String description) {
        return new Category(this.id, this.userId, name, description,
                this.createdAt, LocalDateTime.now());
    }

    public UUID getId() { return id; }
    public UUID getUserId() { return userId; }
    public String getName() { return name; }
    public String getDescription() { return description; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }
}