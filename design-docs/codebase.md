# Politech Codebase Structure

Reference for the current modular layout. The base app is working and already modular.

---

## Top-level layout

```
src/
├── main.rs          # Entry: config, DB/Redis, migrations, router wiring
├── config.rs        # APP_HOST, APP_PORT, DATABASE_URL, REDIS_URL, APP_SECRET_KEY
├── state.rs         # AppState { db, redis, secret_key }
├── models.rs        # Shared: ApiResponse<T>, HealthResponse
├── middleware.rs    # require_auth (x-session-id → AuthenticatedUser), rate_limit_ip (Redis)
├── crypto.rs        # AES-256-GCM encrypt/decrypt for credential secrets
├── modules/         # Domain modules (auth, users, locality, credentials, orchestrator, ingestion)
└── ui/              # Web UI: layout (app_shell), pages (home, dashboard), components
```

- **Routes** are registered in `main.rs`. Public vs protected split: protected routes use `require_auth` and get `AuthenticatedUser` via `Extension`.
- **Global middleware:** `rate_limit_ip` (then CORS, Trace). Protected subtree has `require_auth`.

---

## Module pattern (per domain)

Each module under `src/modules/<name>/` follows the same shape:

| Layer    | Role |
|----------|------|
| `mod.rs` | Re-exports `handler`, `models`; lists submodules (dal, service, views). |
| `handler.rs` | Axum handlers: extract State/Extension, call service/dal, return `Json<ApiResponse<T>>` or error. |
| `service.rs` | Business logic (hashing, encryption, expiry, selection rules). |
| `dal.rs` | SQLx queries: no business rules, only DB read/write. |
| `models.rs` | Request/response DTOs and internal row types for this module. |
| `views/` | (Optional) Leptos or server-rendered view helpers; can stay empty. |

**Dependencies:** Handlers → service, dal, models. Service may use crypto or shared helpers. DAL only touches `sqlx` and module/global state (e.g. `AppState`).

---

## Modules (current)

| Module        | Purpose | Key routes |
|---------------|--------|------------|
| **auth**      | Register, login, logout, refresh; password hash (placeholder), session expiry. | `POST /api/auth/register`, `login`, `logout`, `refresh` |
| **users**     | List sessions for current user. | `GET /api/users/me/sessions` (protected) |
| **locality**  | Philippine locality hierarchy (region → barangay). | `GET /api/localities` (public) |
| **credentials** | Provider credentials: encrypted secret (AES-256-GCM), health row, CRUD, disable, rotate. Role checks: admin/analyst for write, admin for disable/rotate. | `GET|POST /api/credentials`, `PATCH|POST .../:id/disable|rotate` (protected) |
| **orchestrator** | Skeleton: list orchestration runs. | `GET /api/orchestrator/runs` (protected) |
| **ingestion** | Source config CRUD, ingest job queue, raw_ingest_event; trigger job. | `GET|POST /api/sources`, `GET /api/admin/ingest/jobs`, `POST /internal/ingest/run/:source_id` (protected) |
| **topics** | User topics with traits (jsonb); AI-derived traits; used for alignment. | `GET|POST /api/topics`, `GET /api/topics/:id` (public) |
| **locations** | Philippine regions (and later lower levels); geojson_key matches GeoJSON adm1_en. | `GET /api/locations`, `GET /api/locations/:id` (public) |
| **maps** | Alignment (1–10) per region for a topic; feeds sentiment map. | `GET /api/map/alignment/:topic_id` (public) |

---

## UI: SSR pages + WASM reactivity at /app

**SSR pages (`/`, `/dashboard`):** Plain HTML from `page_shell()`, full page loads, no client-side reactivity. **WASM reactivity (`/app/`):** Leptos CSR app in `src/lib.rs` (feature `client`), built with Trunk, served from `dist/` at **/app/** — reactive counter and fetch to APIs. Build: `trunk build index.html`; server mounts `dist/` at `/app`.

**Extend:** Add reactive components in `src/lib.rs` (or a module under `#[cfg(feature = "client")]`); later you can use Leptos SSR + hydration for shared server/client components.

---

- **Pages:** `ui/pages/home.rs`, `ui/pages/dashboard.rs` — each exports an `async fn` that returns `impl IntoResponse` (HTML). They use `ui::layout::app_shell::page_shell(title, subtitle, body_html)` to wrap raw HTML in the Salesforce CRM–style shell.
- **Layout:** `ui/layout/app_shell.rs` — `AppShell` Leptos component (for future full Leptos SSR/hydration) and `page_shell()` for current server-rendered HTML.
- **Styles:** `public/styles.css` (Tailwind + Salesforce-like tokens: `sf-card`, `sf-nav-link`, `sf-base`, `sf-text`, etc.). Source: `web/styles/input.css` when using Tailwind CLI.

---

## Migrations

- `202602250001_phase1_foundation.sql` — `app_user`, `user_session`, `locality`.
- `202602250002_locality_seed.sql` — Seed NCR / Manila / District 1 / Barangay 699.
- `202602250003_phase1_credentials_orchestrator_ingestion.sql` — `credential`, `credential_session`, `credential_health`; `orchestration_run`, `orchestration_chunk`, `orchestration_usage`; `source_config`, `ingest_job`, `raw_ingest_event`; `abuse_action`.

---

## Adding a new module

1. Create `src/modules/<name>/` with `mod.rs`, `handler.rs`, `service.rs`, `dal.rs`, `models.rs` (and optionally `views/`).
2. In `src/modules/mod.rs`: `pub mod <name>;`.
3. In `main.rs`: add routes; if protected, attach them under the `protected` router (so they get `require_auth`).
4. Add any new tables in a new migration under `migrations/`.

This keeps the base app consistent and modular for Phase 2+ (trends, groups, alignment maps, simulations, etc.).
