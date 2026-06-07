package dev.mariinkys.cocookies.infrastructure.persistence.entity;

import jakarta.persistence.*;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

@Entity
@Table(name = "recipes")
public class RecipeJpaEntity {

    @Id
    @Column(columnDefinition = "TEXT", updatable = false, nullable = false)
    private String id;

    @Column(name = "user_id", columnDefinition = "TEXT", nullable = false)
    private String userId;

    @ManyToOne(fetch = FetchType.EAGER)
    @JoinColumn(name = "category_id")
    private CategoryJpaEntity category;

    @ManyToOne(fetch = FetchType.EAGER)
    @JoinColumn(name = "difficulty_id")
    private DifficultyJpaEntity difficulty;

    @Column(nullable = false, length = 255)
    private String title;

    @Column(columnDefinition = "TEXT")
    private String description;

    @Column(name = "prep_time")
    private Integer prepTime;

    @Column(name = "cook_time")
    private Integer cookTime;

    @Column(nullable = false)
    private int servings;

    @Column(name = "image_url", length = 500)
    private String imageUrl;

    @Column(nullable = false, columnDefinition = "INTEGER")
    private boolean shared;

    @Column(name = "created_at", columnDefinition = "TEXT", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @Column(name = "updated_at", columnDefinition = "TEXT", nullable = false)
    private LocalDateTime updatedAt;

    @OneToMany(mappedBy = "recipe", fetch = FetchType.EAGER,
            cascade = CascadeType.ALL, orphanRemoval = true)
    @OrderBy("stepNumber ASC")
    private Set<RecipeStepJpaEntity> steps = new HashSet<>();

    @OneToMany(mappedBy = "recipe", fetch = FetchType.EAGER,
            cascade = CascadeType.ALL, orphanRemoval = true)
    @OrderBy("sortOrder ASC")
    private Set<RecipeIngredientJpaEntity> ingredients = new HashSet<>();

    @OneToOne(mappedBy = "recipe", fetch = FetchType.EAGER,
            cascade = CascadeType.ALL, orphanRemoval = true)
    private RecipeNutritionJpaEntity nutrition;

    @PrePersist
    protected void onCreate() {
        createdAt = LocalDateTime.now();
        updatedAt = LocalDateTime.now();
    }

    @PreUpdate
    protected void onUpdate() {
        updatedAt = LocalDateTime.now();
    }

    protected RecipeJpaEntity() {}

    public RecipeJpaEntity(String id, String userId, CategoryJpaEntity category,
                           DifficultyJpaEntity difficulty, String title, String description,
                           Integer prepTime, Integer cookTime, int servings, String imageUrl,
                           boolean shared, LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.userId = userId;
        this.category = category;
        this.difficulty = difficulty;
        this.title = title;
        this.description = description;
        this.prepTime = prepTime;
        this.cookTime = cookTime;
        this.servings = servings;
        this.imageUrl = imageUrl;
        this.shared = shared;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public String getId() { return id; }
    public String getUserId() { return userId; }
    public CategoryJpaEntity getCategory() { return category; }
    public DifficultyJpaEntity getDifficulty() { return difficulty; }
    public String getTitle() { return title; }
    public String getDescription() { return description; }
    public Integer getPrepTime() { return prepTime; }
    public Integer getCookTime() { return cookTime; }
    public int getServings() { return servings; }
    public String getImageUrl() { return imageUrl; }
    public boolean isShared() { return shared; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    public Set<RecipeStepJpaEntity> getSteps() { return steps; }
    public void setSteps(Set<RecipeStepJpaEntity> steps) { this.steps = steps; }

    public Set<RecipeIngredientJpaEntity> getIngredients() { return ingredients; }
    public void setIngredients(Set<RecipeIngredientJpaEntity> ingredients) { this.ingredients = ingredients; }

    public RecipeNutritionJpaEntity getNutrition() { return nutrition; }
    public void setNutrition(RecipeNutritionJpaEntity nutrition) { this.nutrition = nutrition; }
}