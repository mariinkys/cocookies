package dev.mariinkys.cocookies.domain.models;

public class RecipeNutrition {

    private Long id;
    private final Double servingSizeValue;
    private final String servingSizeUnit;

    private final Double calories;
    private final Double proteinG;
    private final Double carbsG;
    private final Double sugarG;
    private final Double fatG;
    private final Double saturatedFatG;
    private final Double fiberG;
    private final Double sodiumMg;

    // For creating a new nutrition entry
    public RecipeNutrition(Double servingSizeValue, String servingSizeUnit,
                           Double calories, Double proteinG, Double carbsG, Double sugarG,
                           Double fatG, Double saturatedFatG, Double fiberG, Double sodiumMg) {
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

    // For reconstructing from DB
    public RecipeNutrition(Long id, Double servingSizeValue, String servingSizeUnit,
                           Double calories, Double proteinG, Double carbsG, Double sugarG,
                           Double fatG, Double saturatedFatG, Double fiberG, Double sodiumMg) {
        this(servingSizeValue, servingSizeUnit,
                calories, proteinG, carbsG, sugarG, fatG, saturatedFatG, fiberG, sodiumMg);
        this.id = id;
    }

    public Long getId()                { return id; }
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