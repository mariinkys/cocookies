package dev.mariinkys.cocookies.domain.models;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public class Recipe {

    private UUID id;
    private final UUID userId;
    private final Category category;
    private final Difficulty difficulty;
    private final String title;
    private final String description;
    private final Integer prepTime;
    private final Integer cookTime;
    private final int servings;
    private final String imageUrl;
    private final boolean shared;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    private final List<RecipeStep> steps;
    private final List<RecipeIngredient> ingredients;
    private final RecipeNutrition nutrition;

    // For creating a new recipe
    public Recipe(UUID userId, Category category, Difficulty difficulty, String title,
                  String description, Integer prepTime, Integer cookTime, int servings,
                  String imageUrl, boolean shared,
                  List<RecipeStep> steps, List<RecipeIngredient> ingredients,
                  RecipeNutrition nutrition) {
        if (steps == null || steps.isEmpty())
            throw new IllegalArgumentException("A recipe must have at least one step");
        if (ingredients == null || ingredients.isEmpty())
            throw new IllegalArgumentException("A recipe must have at least one ingredient");

        this.id = UUID.randomUUID();
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
        this.steps = List.copyOf(steps);
        this.ingredients = List.copyOf(ingredients);
        this.nutrition = nutrition;
    }

    // For reconstructing from DB
    public Recipe(UUID id, UUID userId, Category category, Difficulty difficulty, String title,
                  String description, Integer prepTime, Integer cookTime, int servings,
                  String imageUrl, boolean shared, List<RecipeStep> steps,
                  List<RecipeIngredient> ingredients, RecipeNutrition nutrition,
                  LocalDateTime createdAt, LocalDateTime updatedAt) {
        this(userId, category, difficulty, title, description, prepTime, cookTime,
                servings, imageUrl, shared, steps, ingredients, nutrition);

        this.id = id;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public Recipe withUpdatedDetails(Category category, Difficulty difficulty, String title,
                                     String description, Integer prepTime, Integer cookTime,
                                     int servings, String imageUrl, boolean shared,
                                     List<RecipeStep> steps, List<RecipeIngredient> ingredients,
                                     RecipeNutrition nutrition) {
        return new Recipe(this.id, this.userId, category, difficulty, title, description,
                prepTime, cookTime, servings, imageUrl, shared, steps, ingredients, nutrition,
                this.createdAt, LocalDateTime.now());
    }

    public UUID getId() { return id; }
    public UUID getUserId() { return userId; }
    public Category getCategory() { return category; }
    public Difficulty getDifficulty() { return difficulty; }
    public String getTitle() { return title; }
    public String getDescription() { return description; }
    public Integer getPrepTime() { return prepTime; }
    public Integer getCookTime() { return cookTime; }
    public int getServings() { return servings; }
    public String getImageUrl() { return imageUrl; }
    public boolean isShared() { return shared; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }
    public List<RecipeStep> getSteps() { return steps; }
    public List<RecipeIngredient> getIngredients() { return ingredients; }
    public RecipeNutrition getNutrition() { return nutrition; }
}