package dev.mariinkys.cocookies.domain.models;

public class RecipeStep {

    private Long id;
    private final int stepNumber;
    private final String instructions;
    private final Integer duration;

    // For creating a new step
    public RecipeStep(int stepNumber, String instructions, Integer duration) {
        this.stepNumber = stepNumber;
        this.instructions = instructions;
        this.duration = duration;
    }

    // For reconstructing from DB
    public RecipeStep(Long id, int stepNumber, String instructions, Integer duration) {
        this(stepNumber, instructions, duration);
        this.id = id;
    }

    public Long getId() { return id; }
    public int getStepNumber() { return stepNumber; }
    public String getInstructions() { return instructions; }
    public Integer getDuration() { return duration; }
}