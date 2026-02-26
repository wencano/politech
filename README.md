# Politech

Geospatial political intelligence and simulation platform for the Philippines.
Modular monolith built on Leptos + Axum (Rust), PostgreSQL (PostGIS + pgvector), Redis, and Tailwind CSS with a Salesforce CRM-inspired design system.

## Requirements

| Tool | Version |
|------|---------|
| Rust | 1.93.1 (see `rust-toolchain.toml`) |
| PostgreSQL | 14+ with `uuid-ossp` extension |
| Redis | 6+ |
| Node.js | 18+ (for Tailwind CSS build only) |

## Quick start

### 1. Database

```bash
createdb politech
psql politech -c 'CREATE EXTENSION IF NOT EXISTS "uuid-ossp";'
```

### 2. Environment

```bash
cp .env.example .env
```

Edit `.env`:

```env
DATABASE_URL=postgres://<user>:<pass>@localhost:5432/politech
REDIS_URL=redis://127.0.0.1:6379
# Generate a secret key for credential encryption:
APP_SECRET_KEY=$(openssl rand -hex 32)
```

### 3. CSS (optional — prebuilt file already included)

```bash
npm install
npm run css:build
```

### 4. Run

```bash
cargo run
```

The server starts on `http://0.0.0.0:3000` by default.
Migrations run automatically on startup.

---

## API reference (Phase 1)

All protected routes require the `x-session-id` header obtained from login/register.

### Public

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Home page |
| `GET` | `/dashboard` | Dashboard shell |
| `GET` | `/health` | Health check |

### Auth — `/api/auth/*`

| Method | Path | Body / Headers | Description |
|--------|------|----------------|-------------|
| `POST` | `/api/auth/register` | `{email, password}` | Create account + session |
| `POST` | `/api/auth/login` | `{email, password}` | Login, returns session |
| `POST` | `/api/auth/logout` | `x-session-id` | Revoke session |
| `POST` | `/api/auth/refresh` | `x-session-id` | Rotate session token |

Response shape: `{"ok": true, "data": {"user_id": "…", "session_id": "…", "role": "user"}}`

### Users & Sessions — protected

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/users/me/sessions` | List your sessions |

### Locality

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/localities` | List locality hierarchy (region → barangay) |

### Credentials — protected, admin/analyst write

| Method | Path | Body | Description |
|--------|------|------|-------------|
| `GET` | `/api/credentials` | — | List credentials (secret never returned) |
| `POST` | `/api/credentials` | `{provider, label, secret, capabilities?, priority_weight?}` | Create credential (AES-256-GCM encrypted) |
| `PATCH` | `/api/credentials/:id` | `{label?, status?, priority_weight?, capabilities?}` | Update metadata |
| `POST` | `/api/credentials/:id/disable` | — | Immediately disable |
| `POST` | `/api/credentials/:id/rotate` | `{secret}` | Replace secret, bump version |

Valid `status` values: `active`, `cooldown`, `rate_limited`, `disabled`, `expired`

### Orchestrator — protected, admin/analyst

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/orchestrator/runs` | List recent orchestration runs |

### Ingestion / Sources — protected

| Method | Path | Body | Description |
|--------|------|------|-------------|
| `GET` | `/api/sources` | — | List source configs |
| `POST` | `/api/sources` | `{name, ingest_mode, base_url, poll_interval_seconds?}` | Register source (`ingest_mode`: `api` \| `scrape`) |
| `GET` | `/api/admin/ingest/jobs` | — | List ingest jobs (admin/analyst) |
| `POST` | `/internal/ingest/run/:source_id` | — | Enqueue ingest job (admin/analyst) |

---

## RBAC roles

| Role | Access |
|------|--------|
| `public` | Unauthenticated routes only |
| `user` | All authenticated routes |
| `analyst` | + credential write, source management, ingest jobs |
| `admin` | + credential disable/rotate |

---

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `APP_HOST` | `0.0.0.0` | Bind address |
| `APP_PORT` | `3000` | Listen port |
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/politech` | PostgreSQL DSN |
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis DSN |
| `APP_SECRET_KEY` | insecure dev key | 64-char hex string (32 bytes) for AES-256-GCM credential encryption. Generate: `openssl rand -hex 32` |
| `RUST_LOG` | `info` | Log filter (e.g. `debug`, `politech=trace`) |

---

## WASM reactivity (Leptos CSR)

A reactive client app (WASM + JS) is available at **/app/** so you can try Leptos reactivity before building other pages.

1. **Build the client** (requires Rust wasm32 target and Trunk):
   ```bash
   rustup target add wasm32-unknown-unknown
   # If NO_COLOR is set (e.g. in IDE), unset it for Trunk: unset NO_COLOR
   trunk build index.html
   ```
   Output goes to `dist/`. The server serves `dist/` at `/app/` (see `main.rs`).

2. **Run the server** and open **http://localhost:3000/app/** in the browser. You get:
   - A **reactive counter** (signal, no full reload).
   - A **Fetch /health** button that calls the backend API and shows the JSON.

3. **Rebuild after editing** the client: `trunk build index.html` (or `trunk watch index.html` during development).

- **Cargo features:** The crate uses `server` (default) and `client`. The server binary is built with `cargo run` (or `--features server`). The WASM lib `politech_app` is built by Trunk with `--no-default-features --features client`. See `index.html` and `Trunk.toml`.

## Development notes

- **UI rendering** — SSR pages (e.g. `/`, `/dashboard`) use `page_shell()` and full reloads. The **WASM app at /app/** is Leptos CSR (reactive). See `design-docs/codebase.md` (§ UI).
- **Migrations** run automatically at startup via `sqlx::migrate!()`. Migration files are in `migrations/`.
- **Password storage** is a Phase 1 placeholder (`plain::` prefix). Replace with Argon2 before production.
- **Sessions** are stateful UUIDs stored in the DB. Pass `x-session-id` in request headers.
- **Rate limiting** — 120 requests/min per IP via Redis. Fails open if Redis is unreachable.
- **CSS** — prebuilt `public/styles.css` is committed so the server works without a Node.js build step. Rebuild with `npm run css:build` after editing `web/styles/input.css`.

---

## Roadmap

| Phase | Status | Focus |
|-------|--------|-------|
| 1 | ✅ Complete | Auth, locality, credentials, orchestrator skeleton, ingestion baseline, rate limiting |
| 2 | Planned | Population traits, sources/topics, ingestion pipeline, groups, alignment maps |
| 3 | Planned | Simulations, alignment engine, persona/policy maps |
| 4 | Planned | Recommendations, chats, live orchestrator provider calls |
