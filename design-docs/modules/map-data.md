# Map Data Module Design

## Purpose
Serve geospatial map layers and geometry used by locality and simulation visualizations. Primary thematic layers are **alignment maps** (topic, persona, policy vs locality population) on a **1–10 dislike–meh–like** scale.

## Responsibilities
- Store and version map geometries.
- Provide map tile or GeoJSON delivery endpoints.
- Support locality boundary overlays and optional custom regions.
- **Alignment map layers:** serve per-locality `alignment_1_10` for topics, political personas, and policies (computed from persona-trait alignment; see Topic Analytics).
- Optional intensity layers for “where is this topic hot” (mention volume).

## Schema (Proposed)
- `map_geometry` (`id`, `locality_id`, `geom`, `source`, `version`, `created_at`)
- `map_layer` (`id`, `key`, `title`, `style_json`, `status`, `updated_at`)
- `alignment_map_metric` (see Topic Analytics: `alignable_type`, `alignable_id`, `locality_id`, `alignment_1_10`, confidence; 1=dislike, 5=meh, 10=like)
- `map_topic_layer_metric` (optional: intensity-only; `topic_id`, `locality_id`, `intensity_score`, etc.)

## Handlers (HTTP/API)
- `GET /api/map/layers`
- `GET /api/map/geometry/:localityId`
- `GET /api/map/tiles/:z/:x/:y`
- `GET /api/map/alignment/:alignableType/:alignableId` — returns locality features with `alignment_1_10` (topic, persona, or policy).
- `GET /api/map/topics/:topicId/layers` — topic alignment layer (and optional intensity).

## Services
- `MapGeometryService`
- `MapLayerService`
- `MapTileService`
- `AlignmentMapLayerService` — builds map features from `alignment_map_metric`; same 1–10 scale for topic, persona, policy.
- `TopicMapLayerService` — topic-specific layer (alignment + optional intensity).

## DAL
- `MapGeometryRepository`
- `MapLayerRepository`
- `AlignmentMapMetricRepository` (read alignment metrics for map rendering)
- `MapTopicLayerMetricRepository` (optional intensity)

## Views
- Interactive map canvas
- Layer toggles (topic / persona / policy alignment layers)
- **Scale legend:** 1 = dislike, 5 = meh, 10 = like
- Boundary detail tooltip (show `alignment_1_10` and optional confidence)
- Optional intensity heatmap (“where is this topic hot”)
