# Locality Traits Extrapolation Module Design

## Purpose
Compute aggregated trait vectors for higher-level localities using barangay snapshots.

## Responsibilities
- Roll up trait vectors from barangay to town/city/district/province/region.
- Track extrapolation method and versioning.
- Recompute aggregates when source snapshots change.

## Schema (Proposed)
- `locality_trait_aggregate` (`id`, `locality_id`, `source_snapshot_version`, `method`, `trait_vector`, `created_at`)
- `extrapolation_run` (`id`, `status`, `scope_json`, `started_at`, `ended_at`)

## Handlers (HTTP/API)
- `GET /api/locality-traits/:id/latest`
- `POST /api/locality-traits/recompute`

## Services
- `TraitExtrapolationService`
- `AggregateRefreshService`

## DAL
- `LocalityTraitAggregateRepository`
- `ExtrapolationRunRepository`

## Views
- Aggregated trait chart
- Recompute status panel (admin)
