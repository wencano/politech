# Politech — Quick Context & High-Level Todo

## Quick context

- **What:** Geospatial political intelligence platform for the Philippines — trending topics by group, persona/policy alignment simulations, alignment maps (1–10 dislike–meh–like), and recommendations.
- **Stack:** Leptos (Rust) web app, Tailwind CSS + Salesforce CRM-inspired UI system, PostgreSQL (PostGIS + pgvector), Redis. Optional Rust/Go workers. Small VM target; account-gated APIs; anti-DDoS/bot/scraper measures.
- **Architecture:** Modular monolith. Alignment = message/persona vector • locality population vector → map and simulation results. Topic, persona, and policy share the same alignment map scale (1–10).
- **Docs:** `prd.md`, `design-docs/modules/*.md`, `design-docs/modules/topic-analytics.md` (KPIs, alignment formulas).

---

## High-level todo

### Phase 1 — Foundation
- [x] Project setup: Leptos app, Postgres (PostGIS + pgvector), Redis, env/config.
- [x] UI baseline: Tailwind setup + Salesforce CRM-inspired tokens/components (shell, nav, card, form, table, badge).
- [x] Auth: register, login, logout, refresh; sessions; RBAC (public, user, analyst, admin) baseline handlers.
- [x] Users & sessions: user/session tables and basic session listing API.
- [x] Locality: hierarchy tables, seed data, and list API baseline.
- [x] Dashboard shell: authenticated layout, nav, placeholder widgets.
- [x] Edge/abuse: rate limits (IP + account), account-gated routes, baseline challenge flow.
- [x] Credentials module: store provider credentials (encrypted), health; admin CRUD.
- [x] Orchestrator skeleton: credential selection, provider adapter interface, no chat yet.
- [x] Ingestion baseline: source config, raw ingest storage, one API or scrape connector.

### Phase 2 — Census map
- [ ] Reference implementation note: Tim Hormigos' Educational Attainment Map (Notion + GIS workflow) with 42k+ barangays, census, and education data; use this as benchmark context for interaction and data density.
- [ ] Reference link: https://tinyurl.com/bnstim-rph-eduk
- [ ] Data source note: use `philatlas.com` for practical barangay-level reference/bootstrap, and prioritize PSA official sources (`psa.gov.ph`, `openstat.psa.gov.ph`) as canonical census data for validation and long-term ingestion.
- [ ] Barangay locations module: canonical barangay geospatial layer and locality join keys.
- [ ] Location hierarchy model: barangay, city/town, province, and region records must include explicit parent fields (parent_locality_id/parent_psgc) to support tree fetching and drill-down.
- [ ] PSA census integration module: ingest/normalize barangay-level census datasets (starting with population totals).
- [ ] Population segments module: define and expose census-based segment buckets per barangay.
- [ ] Voting population module: compute and expose voting-age / voting-eligible population metrics at barangay level.
- [ ] Census data abstraction: keep PSA ingestion separate so additional census datasets can be added without map module rewrites.
- [ ] Census map API + UI layer: render barangay map overlays for population, segments, and voting population.
- [ ] Validation pass: cross-check mapped aggregates against PSA reference totals before simulation-dependent features.

### Phase 3 — AgriMap
- [ ] Agricultural layer module: define agri-specific geospatial layers and locality join model.
- [ ] Agriculture datasets module: ingest/normalize initial agriculture indicators (crop mix, production, land use as available).
- [ ] AgriMap API + UI layer: render barangay-level agri overlays and filters.
- [ ] Extensibility pass: keep agriculture data loaders modular for additional sources and indicators.

### Phase 4 — Trends & groups & maps
- [ ] Population traits: barangay-level trait vectors, versioned snapshots, regeneration job.
- [ ] Locality trait extrapolation: roll-up to town/district/province/region.
- [ ] Sources & topics: source catalog, topic taxonomy, topic resolution from mentions.
- [ ] Ingestion pipeline: topic extraction, sentiment, trend_fact + aggregates; topic_stats_daily, topic_sentiment_daily.
- [ ] Population groups: seeded groups, dynamic group rules, threshold evaluation.
- [ ] Topic analytics: stats/sentiment APIs; alignment_map_metric (topic/persona/policy); 1–10 scale computation.
- [ ] Map data: geometries, layers, alignment map endpoint (alignment_1_10 by locality).
- [ ] Dashboard: trend cards, topic stats/sentiment, alignment map widget (1–10 legend).

### Phase 5 — Simulations
- [ ] Simulations module (deferred until after census map phase): create simulation (persona/policy + scope), run job, status.
- [ ] Simulation results: result snapshots, locality breakdown, alignment scores.
- [ ] Alignment engine: trait vectors, dot product / similarity, 1–10 mapping for maps.
- [ ] Persona & policy map APIs: GET /api/personas/:id/map, GET /api/simulations/:id/map.
- [ ] Dashboard: simulation list, run flow, result view and comparison.

### Phase 6 — Recommendations & chat
- [ ] Recommendation engine: request from simulation/policy, ranked items, rationale; feedback.
- [ ] Chats & chat messages: threads, messages, streaming storage.
- [ ] Orchestrator: route to credential, session refresh, send prompt, stream response; persist assistant message.
- [ ] Dashboard: recommendation request flow, chat UI, notifications (optional).

### Cross-cutting / ongoing
- [ ] Observability: health, logs, metrics, audit for sensitive actions.
- [ ] Admin: sources, groups, credentials, moderation, abuse actions.
- [ ] Caching: Redis for hot reads (trends, topic stats, alignment map).
- [ ] Jobs: scheduler for ingestion, trait regen, extrapolation, aggregate refresh.
- [ ] Docs: keep prd.md and design-docs in sync with implementation.
