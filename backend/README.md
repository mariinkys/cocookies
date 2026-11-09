# Cocookies backend

Rust + [axum](https://github.com/tokio-rs/axum) + SQLite ([sqlx](https://github.com/launchbadge/sqlx)),
with native PDF generation through [krilla](https://github.com/LaurenzV/krilla).

A single ~12 MB binary that idles at roughly 15 MB of RAM.

## Running

```sh
cargo run                      # http://localhost:8080, database in ./cocookies.db
cargo test                     # unit + end-to-end API tests
cargo clippy --all-targets -- -D warnings
```

Migrations in `migrations/` are embedded in the binary and applied on start-up. The first
migration is idempotent, so a database created by the previous (Java) backend is upgraded in
place: just point `DB_URL` at it.

## Configuration

All settings are environment variables (a `.env` file is **not** read automatically).

| Variable | Default | Notes |
|---|---|---|
| `BIND_ADDR` | `0.0.0.0:8080` | |
| `DB_URL` | `cocookies.db` | SQLite file path; `jdbc:sqlite:` / `sqlite://` prefixes accepted |
| `DB_MAX_CONNECTIONS` | `4` | |
| `JWT_SECRET` | dev secret (warns) | **Set it.** ≥ 32 random bytes |
| `JWT_EXPIRATION_SECONDS` | `10800` | access token lifetime (3 h) |
| `REFRESH_TOKEN_DAYS` | `7` | |
| `COOKIE_SECURE` | `false` | `true` when served over HTTPS (also enables HSTS) |
| `CORS_ALLOWED_ORIGIN` | `http://localhost:5173` | only needed when the frontend is on another origin |
| `TRUST_PROXY` | `false` | use `X-Real-IP` / `X-Forwarded-For` for rate limiting; enable behind the bundled nginx |
| `GUEST_ACCESS` | `true` | let visitors browse shared recipes read-only without an account |
| `MAX_LOGIN_ATTEMPTS` / `LOCKOUT_DURATION_MINUTES` | `5` / `15` | account lock-out |
| `AUTH_RATE_LIMIT_PER_MINUTE` | `5` | login + register requests per IP |
| `BCRYPT_COST` | `10` | |
| `OLLAMA_URL` / `OLLAMA_MODEL` | `http://localhost:11434` / `gemma3:4b` | AI import |
| `OLLAMA_TIMEOUT_SECONDS` / `LLM_MAX_CONCURRENCY` | `300` / `2` | |
| `RUST_LOG` | `info,sqlx=warn` | log filter |

## Architecture

The code follows Clean Architecture, in the layout of
[axum-clean-architecture](https://github.com/Thodin/axum-clean-architecture). Dependencies only
point inwards:

```
  infra  ──►  adapters  ──►  application  ──►  domain
(wiring)   (http, sqlite,    (use cases +     (entities,
            crypto, pdf…)     their ports)      rules)
```

```
src/
├── main.rs                    runtime + server start-up
├── lib.rs                     layer map; re-exports `application::*` and `domain::*`
├── domain/                    plain Rust: no axum, sqlx, serde…
│   ├── entities/              user, session, taxonomy, recipe, imported_recipe, page
│   ├── validation.rs          rule helpers (entities validate themselves)
│   └── time.rs                ISO-8601 timestamps
├── application/
│   ├── app_error.rs           framework-free error type
│   └── use_cases/             one `XxxUseCases` per feature + the traits (ports) it needs
│       ├── auth.rs            login, lock-out, refresh rotation   (SessionPersistence, AccessTokenService, …)
│       ├── user.rs            account management                  (UserPersistence, PasswordHasher)
│       ├── taxonomy.rs        categories & difficulties           (TaxonomyPersistence)
│       ├── recipe.rs          CRUD, sharing, PDF                  (RecipePersistence, RecipeRenderer, RemoteFetcher)
│       ├── recipe_import.rs   AI import                           (RecipeExtractor, DocumentReader)
│       ├── system.rs          version, health                     (StorageMaintenance)
│       └── fakes.rs           in-memory ports for unit tests
├── adapters/                  implementations of the ports
│   ├── http/                  axum: app_state (FromRef per use case), app_error_impl,
│   │   │                      extractors, cookies, rate_limit, pagination
│   │   └── routes/            one file per feature: route table + DTOs + handlers
│   ├── persistence/           SqlitePersistence: row structs + SQL per aggregate
│   ├── crypto/                bcrypt, JWT, refresh tokens
│   ├── pdf/                   krilla renderer (layout engine + templates)
│   ├── llm/                   Ollama client + lenient JSON parsing
│   ├── documents/             PDF / HTML → text
│   └── net/                   SSRF-safe fetcher
└── infra/
    ├── config.rs  db.rs       environment + SQLite bootstrap/migrations
    ├── setup.rs               composition root: builds adapters, injects them into use cases
    ├── app.rs                 router + global middleware (CORS, security headers, tracing)
    └── jobs.rs                hourly housekeeping
```

**Adding a feature:** add entities in `domain/entities`, a `use_cases/<feature>.rs` with its
`XxxUseCases` and the port traits it needs, implement those ports in `adapters/` (usually
`persistence/<feature>.rs`), add `adapters/http/routes/<feature>.rs` with its DTOs, then wire
it in `infra/setup.rs` and `AppState`.

**Testing:** use cases are unit-tested against the in-memory fakes in `use_cases/fakes.rs`;
adapters have focused unit tests; `tests/api.rs` drives the real router on a real SQLite file.

## API

Same routes and JSON shapes as before, with these differences:

* **Guests**: `GET /api/recipes/shared`, `GET /api/recipes/{id}` and
  `GET /api/recipes/{id}/pdf/{template}` work without signing in for shared recipes
  (when `GUEST_ACCESS=true`). `GET /api/system/config` tells the client whether that is enabled.
* **List endpoints** (`/api/recipes`, `/api/recipes/shared`) return lightweight recipe *cards*
  (no steps / ingredients / nutrition; category and difficulty as `{id, name}`).
* Only the **owner** can modify or delete a recipe, even a shared one; recipes can only reference
  the owner's own categories/difficulties.
* Proper status codes for not found (404), duplicates (409) and bad references (400) instead of
  generic 500s. `GET /api/users` (list every user) was removed.
* Sessions are bound to the user id, so changing your e-mail no longer logs you out. Refresh
  tokens are stored hashed, rotated on use, and reuse revokes all of the user's sessions.
* `GET /api/system/health` for container health checks.

## Fonts

PDFs use [Gelasio](https://github.com/SorkinType/Gelasio) (SIL Open Font License, see
`assets/fonts/OFL.txt`), metric-compatible with Georgia used by the original template.
