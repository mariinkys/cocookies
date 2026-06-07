CREATE TABLE users (
    id                    TEXT    NOT NULL PRIMARY KEY,  -- UUID stored as TEXT
    name                  TEXT    NOT NULL,
    email                 TEXT    NOT NULL UNIQUE,
    password              TEXT    NOT NULL,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until          TEXT,                          -- nullable ISO-8601 timestamp
    created_at            TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at            TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE refresh_tokens (
    id         TEXT    NOT NULL PRIMARY KEY,
    token      TEXT    NOT NULL UNIQUE,
    user_email TEXT    NOT NULL,
    expires_at TEXT    NOT NULL,              -- ISO-8601 timestamp
    revoked    INTEGER NOT NULL DEFAULT 0,    -- boolean: 0 = false, 1 = true
    created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE categories (
    id          TEXT    NOT NULL PRIMARY KEY,
    user_id     TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT,
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (user_id, name)  -- name unique per user, not globally
);

CREATE TABLE difficulties (
    id          TEXT    NOT NULL PRIMARY KEY,
    user_id     TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT,
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (user_id, name)  -- name unique per user, not globally
);

CREATE TABLE recipes (
    id            TEXT    NOT NULL PRIMARY KEY,
    user_id       TEXT    NOT NULL,
    category_id   TEXT,
    difficulty_id TEXT,
    title         TEXT    NOT NULL,
    description   TEXT,
    prep_time     INTEGER,
    cook_time     INTEGER,
    servings      INTEGER NOT NULL DEFAULT 1,
    image_url     TEXT,
    shared        INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at    TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (user_id)       REFERENCES users(id)        ON DELETE CASCADE,
    FOREIGN KEY (category_id)   REFERENCES categories(id)   ON DELETE SET NULL,
    FOREIGN KEY (difficulty_id) REFERENCES difficulties(id) ON DELETE SET NULL
);

CREATE TABLE recipe_steps (
    id           INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id    TEXT    NOT NULL,
    step_number  INTEGER NOT NULL CHECK (step_number >= 1),
    instructions TEXT    NOT NULL,
    duration     INTEGER,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    UNIQUE (recipe_id, step_number)
);

CREATE TABLE recipe_ingredients (
    id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id  TEXT    NOT NULL,
    name       TEXT    NOT NULL,
    quantity   REAL,
    unit       TEXT,
    notes      TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
);

CREATE TABLE recipe_nutrition (
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id           TEXT    NOT NULL UNIQUE,
    serving_size_value  REAL    NOT NULL DEFAULT 100.0,  -- e.g. 100
    serving_size_unit   TEXT    NOT NULL DEFAULT 'g',    -- e.g. 'g', 'ml', 'oz'
    -- All values are per serving_size_value of serving_size_unit
    calories        REAL,  -- kcal
    protein_g       REAL,
    carbs_g         REAL,
    sugar_g         REAL,
    fat_g           REAL,
    saturated_fat_g REAL,
    fiber_g         REAL,
    sodium_mg       REAL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
);


CREATE INDEX idx_users_email                  ON users(email);
CREATE INDEX idx_refresh_tokens_token         ON refresh_tokens(token);
CREATE INDEX idx_refresh_tokens_user_email    ON refresh_tokens(user_email);
CREATE INDEX idx_categories_user_id           ON categories(user_id);
CREATE INDEX idx_difficulties_user_id         ON difficulties(user_id);
CREATE INDEX idx_recipes_user_id              ON recipes(user_id);
CREATE INDEX idx_recipes_category_id          ON recipes(category_id);
CREATE INDEX idx_recipes_difficulty_id        ON recipes(difficulty_id);
CREATE INDEX idx_recipe_steps_recipe_id       ON recipe_steps(recipe_id);
CREATE INDEX idx_recipe_ingredients_recipe_id ON recipe_ingredients(recipe_id);
CREATE INDEX idx_recipe_nutrition_recipe_id   ON recipe_nutrition(recipe_id);

INSERT INTO users (id, name, email, password)
VALUES (
   '00000000-0000-0000-0000-000000000001',
   'Admin',
   'admin@example.com',
   '$2y$10$Dm42cN7wQvU5L9fhOP4E5uOyogeKOvq4i7nuhhHcdPBW8ZD5Fd/0q'
);