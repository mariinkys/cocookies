package dev.mariinkys.cocookies.domain.models;

public class RecipeIngredient {

    private Long id;
    private final String name;
    private final Double quantity; // nullable
    private final String unit;     // nullable, e.g. "grams", "cups"
    private final String notes;    // nullable, e.g. "finely chopped"
    private final int sortOrder;

    // For creating a new ingredient
    public RecipeIngredient(String name, Double quantity, String unit,
                            String notes, int sortOrder) {
        this.name = name;
        this.quantity = quantity;
        this.unit = unit;
        this.notes = notes;
        this.sortOrder = sortOrder;
    }

    // For reconstructing from DB
    public RecipeIngredient(Long id, String name, Double quantity, String unit,
                            String notes, int sortOrder) {
        this(name, quantity, unit, notes, sortOrder);
        this.id = id;
    }

    public Long getId() { return id; }
    public String getName() { return name; }
    public Double getQuantity() { return quantity; }
    public String getUnit() { return unit; }
    public String getNotes() { return notes; }
    public int getSortOrder() { return sortOrder; }
}