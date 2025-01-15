ALTER TABLE recipe_ingredients RENAME TO recipe_ingredients_old;

CREATE TABLE recipe_ingredients (
    recipe_ingredient_id INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL,
    ingredient_name TEXT NOT NULL,
    quantity TEXT NULL, 
    unit TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recipe_id) REFERENCES recipes(recipe_id) ON DELETE CASCADE
);

INSERT INTO recipe_ingredients (
    recipe_ingredient_id, recipe_id, ingredient_name, quantity, unit, created_at, updated_at
)
SELECT 
    recipe_ingredient_id, recipe_id, ingredient_name, CAST(quantity AS TEXT), unit, created_at, updated_at
FROM recipe_ingredients_old;

DROP TABLE recipe_ingredients_old;
