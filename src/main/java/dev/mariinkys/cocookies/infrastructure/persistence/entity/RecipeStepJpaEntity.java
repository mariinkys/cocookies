package dev.mariinkys.cocookies.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "recipe_steps")
public class RecipeStepJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(name = "id", columnDefinition = "INTEGER")
    private Long id;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "recipe_id", nullable = false)
    private RecipeJpaEntity recipe;

    @Column(name = "step_number", nullable = false)
    private int stepNumber;

    @Column(nullable = false, columnDefinition = "TEXT")
    private String instructions;

    @Column
    private Integer duration;

    protected RecipeStepJpaEntity() {}

    public RecipeStepJpaEntity(RecipeJpaEntity recipe,
                               int stepNumber, String instructions, Integer duration) {
        this.recipe = recipe;
        this.stepNumber = stepNumber;
        this.instructions = instructions;
        this.duration = duration;
    }

    public Long getId() { return id; }
    public RecipeJpaEntity getRecipe() { return recipe; }
    public int getStepNumber() { return stepNumber; }
    public String getInstructions() { return instructions; }
    public Integer getDuration() { return duration; }
}