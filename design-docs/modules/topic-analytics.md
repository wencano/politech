# Topic Analytics Module Design

## Purpose
Define unified topic analytics contracts and KPI formulas used by `topics`, `dashboard`, and `map-data` modules.

## Responsibilities
- Standardize topic KPI definitions and computation windows.
- Produce read models for topic stats, sentiment trends, and locality map metrics.
- Keep formulas consistent across APIs, dashboards, and exports.

## Module Boundaries
- This module is an analytics contract and computation layer.
- It reads from ingestion/trend fact tables and publishes stable outputs.
- It does not own UI rendering directly; it serves data to dashboard/topics/map modules.

## Inputs
- `topic_mention`
- `trend_fact`
- `topic_sentiment_fact`
- `source` metadata (trust scores, source type)
- `locality` hierarchy metadata

## Outputs
- `topic_stats_daily`
- `topic_sentiment_daily`
- `topic_map_metric_daily`

## Canonical KPI Definitions

### 1) Mention Volume
- **Definition:** Total valid mentions for a topic within a time bucket.
- **Formula:** `mention_count = count(valid_mentions)`

### 2) Source Count
- **Definition:** Number of distinct sources mentioning a topic in a bucket.
- **Formula:** `source_count = count(distinct source_id)`

### 3) Growth Rate
- **Definition:** Relative change versus previous equivalent bucket.
- **Formula:** `growth_rate = (current_count - prev_count) / nullif(prev_count, 0)`
- **Fallback:** If previous bucket is zero, mark as `null` and set `growth_flag = new_topic_spike`.

### 4) Confidence Score
- **Definition:** Weighted confidence combining extraction certainty and source trust.
- **Formula (v1):**
  - `confidence_score = clamp01(sum(mention_confidence * source_trust_weight) / mention_count)`

### 5) Sentiment Mix
- **Definition:** Proportion of positive/neutral/negative mentions.
- **Formula:**
  - `positive_ratio = positive_count / total_count`
  - `neutral_ratio = neutral_count / total_count`
  - `negative_ratio = negative_count / total_count`

### 6) Sentiment Score (mention-level)
- **Definition:** Single scalar sentiment index for time-series and tables (not map).
- **Formula (v1):**
  - `sentiment_score = (positive_count - negative_count) / nullif(total_count, 0)`
  - Range `[-1, 1]`

### 7) Alignment Map Score (1–10 dislike–meh–like)
- **Definition:** Per-locality resonance between an **alignable** (topic, political persona, or policy) and the locality’s population trait vector. Drives the thematic map so users see *distribution of alignment* (who “likes” vs “dislikes” vs “meh” by place).
- **Alignables:** Topics, political personas, and policies each have (or are reduced to) a trait/message vector. The same alignment pipeline and scale apply to all three.
- **Internal formula (v1):**
  - `resonance_raw = dot(message_vector, locality_population_vector)` or weighted cosine similarity.
  - Internal range typically `[-1, 1]` or `[0, 1]` depending on engine.
- **Display scale (1–10):**
  - `alignment_1_10 = round(1 + 9 * (resonance_raw + 1) / 2)` when internal range is `[-1, 1]`.
  - **1** = dislike / low alignment  
  - **5** = meh / neutral  
  - **10** = like / high alignment  
- Map layers show this value per locality (optionally with confidence); same scale for topic, persona, and policy maps so sentiment/alignment is comparable.

### 8) Locality Intensity Score (optional, non-alignment)
- **Definition:** Topic *activity* intensity for a locality (mention volume share). Use when showing “where is this topic hot” rather than “where does this align.”
- **Formula (v1):**
  - `raw_intensity = locality_topic_mentions / nullif(locality_total_mentions, 0)`
  - `intensity_score = percentile_rank(raw_intensity within peer localities)`
  - Range `[0, 1]`

## Schema (Proposed)

### `topic_stats_daily`
- `id` (bigserial, pk)
- `topic_id` (uuid, indexed)
- `bucket_date` (date, indexed)
- `mention_count` (int)
- `source_count` (int)
- `growth_rate` (numeric, nullable)
- `growth_flag` (text, nullable)
- `confidence_score` (numeric)
- Unique key: (`topic_id`, `bucket_date`)

### `topic_sentiment_daily`
- `id` (bigserial, pk)
- `topic_id` (uuid, indexed)
- `bucket_date` (date, indexed)
- `positive_count` (int)
- `neutral_count` (int)
- `negative_count` (int)
- `sentiment_score` (numeric)
- Unique key: (`topic_id`, `bucket_date`)

### `alignment_map_metric`
- **Purpose:** One table for topic, persona, and policy map layers; alignment computed from trait vectors.
- `id` (bigserial, pk)
- `alignable_type` (enum: topic, persona, policy)
- `alignable_id` (uuid; topic_id, persona_id, or policy/simulation_id)
- `locality_id` (uuid, indexed)
- `bucket_date` (date, nullable; optional time slice)
- `alignment_internal` (numeric; raw resonance e.g. [-1, 1])
- `alignment_1_10` (smallint; 1=dislike, 5=meh, 10=like)
- `confidence_score` (numeric)
- `trait_snapshot_version` (bigint, nullable; for reproducibility)
- Unique key: (`alignable_type`, `alignable_id`, `locality_id`, `bucket_date`)

### `topic_map_metric_daily` (legacy / optional)
- Retain for **intensity-only** layers (where topic is “hot”) if needed.
- `topic_id`, `locality_id`, `bucket_date`, `intensity_score`, `sentiment_score`, `confidence_score`.
- For **alignment** topic maps, use `alignment_map_metric` with `alignable_type = topic`.

## Handlers (HTTP/API Contract)
- `GET /api/topics/:id/stats`
  - Returns KPI time series and latest snapshot.
- `GET /api/topics/:id/sentiment`
  - Returns sentiment counts/ratios and trend line.
- `GET /api/topics/:id/map`
  - Returns **alignment** map: locality-level `alignment_1_10` (and optional confidence) for this topic’s trait vector vs locality populations. Same 1–10 scale as persona/policy maps.
- `GET /api/personas/:id/map`
  - Alignment map for a political persona.
- `GET /api/policies/:id/map` (or `GET /api/simulations/:id/map`)
  - Alignment map for a policy/message (or simulation).
- `GET /api/dashboard/topics/:id/overview`
  - Returns combined payload for dashboard cards/charts/map.

## Services
- `TopicAnalyticsAggregationService`
  - Builds daily/hourly aggregates from fact tables.
- `TopicStatsService`
  - Serves stats with filter/window controls.
- `TopicSentimentService`
  - Serves sentiment trends and summary mix.
- `TopicMapMetricsService`
  - Serves map-layer metrics by locality (legacy intensity if used).
- `AlignmentMapService`
  - Computes and serves alignment map metrics for topic, persona, and policy; returns `alignment_1_10` (1–10 dislike–meh–like) per locality.
- `TopicAnalyticsCacheService`
  - Caches hot queries in Redis with stale-while-revalidate.

## DAL
- `TopicStatsDailyRepository`
- `TopicSentimentDailyRepository`
- `TopicMapMetricDailyRepository`
- `AlignmentMapMetricRepository`
- `TopicAnalyticsQueryRepository` (joined projections for overview endpoint)

## Processing and Refresh Strategy
- Incremental refresh every 5-15 minutes for near-real-time dashboards.
- Full daily consolidation job for finalized daily KPIs.
- Backfill job for replay/recompute when ingestion logic changes.
- Idempotent upserts keyed by `(topic_id, bucket_date[, locality_id])`.

## Caching Strategy
- Cache key pattern:
  - `topic:stats:{topicId}:{window}:{filtersHash}`
  - `topic:sentiment:{topicId}:{window}:{filtersHash}`
  - `topic:map:{topicId}:{bucketDate}:{localityLevel}:{filtersHash}`
- TTL:
  - 1-5 minutes for active topics
  - 10-30 minutes for cold topics

## Views and UX Contracts
- **Topic detail page:** stat cards, trend sparkline, sentiment stacked chart, **alignment map** (1–10 scale).
- **Persona / policy (simulation) pages:** same **alignment map** (1–10 dislike–meh–like) for geographic distribution of alignment.
- **Dashboard topic widget:** compact KPI panel + mini alignment map overlay.
- **Map layer panel:** alignment layer (1–10) for topic, persona, or policy; optional intensity layer for “where is this topic hot.”
- **Scale legend:** 1 = dislike, 5 = meh, 10 = like (consistent across topic, persona, and policy maps).

## Data Quality and Guardrails
- Exclude low-confidence mentions below configurable threshold.
- Require minimum mention count before exposing sentiment score.
- Mark sparse localities as low-confidence on maps.
- Include `data_freshness_ts` in responses.

## Observability
- Track aggregation lag, cache hit rate, and API latency.
- Alert if aggregation freshness exceeds SLO threshold.
- Audit formula version used for each output row.

## Versioning
- Include `formula_version` in aggregate tables or metadata.
- Support controlled migrations when KPI definitions evolve.
