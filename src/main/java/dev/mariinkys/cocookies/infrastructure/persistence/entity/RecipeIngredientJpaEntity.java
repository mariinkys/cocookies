package dev.mariinkys.cocookies.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "recipe_ingredients")
public class RecipeIngredientJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(name = "id", columnDefinition = "INTEGER")
    private Long id;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "recipe_id", nullable = false)
    private RecipeJpaEntity recipe;

    @Column(nullable = false, length = 150)
    private String name;

    @Column
    private Double quantity;

    @Column(length = 50)
    private String unit;

    @Column(columnDefinition = "TEXT")
    private String notes;

    @Column(name = "sort_order", nullable = false)
    private int sortOrder;

    protected RecipeIngredientJpaEntity() {}

    public RecipeIngredientJpaEntity(RecipeJpaEntity recipe, String name,
                                     Double quantity, String unit,
                                     String notes, int sortOrder) {
        this.recipe = recipe;
        this.name = name;
        this.quantity = quantity;
        this.unit = unit;
        this.notes = notes;
        this.sortOrder = sortOrder;
    }

    public Long getId() { return id; }
    public RecipeJpaEntity getRecipe() { return recipe; }
    public String getName() { return name; }
    public Double getQuantity() { return quantity; }
    public String getUnit() { return unit; }
    public String getNotes() { return notes; }
    public int getSortOrder() { return sortOrder; }
}