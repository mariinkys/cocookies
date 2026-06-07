package dev.mariinkys.cocookies.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "recipe_nutrition")
public class RecipeNutritionJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(name = "id", columnDefinition = "INTEGER")
    private Long id;

    @OneToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "recipe_id", nullable = false)
    private RecipeJpaEntity recipe;

    @Column(name = "serving_size_value", nullable = false)
    private Double servingSizeValue;

    @Column(name = "serving_size_unit", nullable = false, columnDefinition = "TEXT")
    private String servingSizeUnit;

    @Column
    private Double calories;

    @Column(name = "protein_g")
    private Double proteinG;

    @Column(name = "carbs_g")
    private Double carbsG;

    @Column(name = "sugar_g")
    private Double sugarG;

    @Column(name = "fat_g")
    private Double fatG;

    @Column(name = "saturated_fat_g")
    private Double saturatedFatG;

    @Column(name = "fiber_g")
    private Double fiberG;

    @Column(name = "sodium_mg")
    private Double sodiumMg;

    protected RecipeNutritionJpaEntity() {}

    public RecipeNutritionJpaEntity(RecipeJpaEntity recipe,
                                    Double servingSizeValue, String servingSizeUnit,
                                    Double calories, Double proteinG, Double carbsG,
                                    Double sugarG, Double fatG, Double saturatedFatG,
                                    Double fiberG, Double sodiumMg) {
        this.recipe = recipe;
        this.servingSizeValue = servingSizeValue;
        this.servingSizeUnit = servingSizeUnit;
        this.calories = calories;
        this.proteinG = proteinG;
        this.carbsG = carbsG;
        this.sugarG = sugarG;
        this.fatG = fatG;
        this.saturatedFatG = saturatedFatG;
        this.fiberG = fiberG;
        this.sodiumMg = sodiumMg;
    }

    public Long getId()                { return id; }
    public RecipeJpaEntity getRecipe() { return recipe; }
    public Double getServingSizeValue(){ return servingSizeValue; }
    public String getServingSizeUnit() { return servingSizeUnit; }
    public Double getCalories()        { return calories; }
    public Double getProteinG()        { return proteinG; }
    public Double getCarbsG()          { return carbsG; }
    public Double getSugarG()          { return sugarG; }
    public Double getFatG()            { return fatG; }
    public Double getSaturatedFatG()   { return saturatedFatG; }
    public Double getFiberG()          { return fiberG; }
    public Double getSodiumMg()        { return sodiumMg; }
}