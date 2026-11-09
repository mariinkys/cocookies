-- 1. Refresh tokens are now bound to the user id (not the email, which can change) and only a
--    SHA-256 hash of the token is persisted. Existing tokens are dropped: users simply log in again.
DROP TABLE IF EXISTS refresh_tokens;

CREATE TABLE refresh_tokens (
    id         TEXT    NOT NULL PRIMARY KEY,
    user_id    TEXT    NOT NULL,
    token_hash TEXT    NOT NULL UNIQUE,                 -- hex(sha256(token))
    expires_at TEXT    NOT NULL,                        -- ISO-8601 UTC
    revoked    INTEGER NOT NULL DEFAULT 0,
    created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id    ON refresh_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);

-- 2. Normalise timestamps written by the previous backend ("yyyy-MM-dd HH:mm:ss") to the
--    canonical ISO-8601 UTC form used everywhere else ("yyyy-MM-ddTHH:mm:ssZ"), so they sort and
--    compare correctly as plain strings.
UPDATE users        SET created_at = strftime('%Y-%m-%dT%H:%M:%SZ', created_at) WHERE created_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', created_at) IS NOT NULL;
UPDATE users        SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) WHERE updated_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) IS NOT NULL;
UPDATE users        SET locked_until = strftime('%Y-%m-%dT%H:%M:%SZ', locked_until) WHERE locked_until IS NOT NULL AND locked_until NOT LIKE '%Z';
UPDATE categories   SET created_at = strftime('%Y-%m-%dT%H:%M:%SZ', created_at) WHERE created_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', created_at) IS NOT NULL;
UPDATE categories   SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) WHERE updated_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) IS NOT NULL;
UPDATE difficulties SET created_at = strftime('%Y-%m-%dT%H:%M:%SZ', created_at) WHERE created_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', created_at) IS NOT NULL;
UPDATE difficulties SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) WHERE updated_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) IS NOT NULL;
UPDATE recipes      SET created_at = strftime('%Y-%m-%dT%H:%M:%SZ', created_at) WHERE created_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', created_at) IS NOT NULL;
UPDATE recipes      SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) WHERE updated_at NOT LIKE '%Z' AND strftime('%Y-%m-%dT%H:%M:%SZ', updated_at) IS NOT NULL;

-- 3. Indexes for the hot listing queries (shared feed + per-user lists sorted by title/date).
CREATE INDEX IF NOT EXISTS idx_recipes_shared_title   ON recipes(shared, title COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_recipes_user_title     ON recipes(user_id, title COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_recipe_steps_order     ON recipe_steps(recipe_id, step_number);
CREATE INDEX IF NOT EXISTS idx_recipe_ingredients_ord ON recipe_ingredients(recipe_id, sort_order);
