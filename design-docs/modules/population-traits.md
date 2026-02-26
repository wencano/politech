# Population Traits Module Design

## Purpose
Store barangay-level sample population traits and versioned snapshots used by simulations.

## Responsibilities
- Persist normalized trait vectors per barangay.
- Support trait regeneration runs with version history.
- Provide trait snapshots for simulation requests.

## Schema (Proposed)
- `trait_snapshot` (`id`, `locality_id`, `version`, `trait_vector`, `sample_size`, `created_at`)
- `trait_generation_run` (`id`, `status`, `input_ref`, `started_at`, `ended_at`, `created_by`)

## Handlers (HTTP/API)
- `GET /api/traits/locality/:id/latest`
- `GET /api/traits/locality/:id/history`
- `POST /api/traits/regenerate`

## Services
- `TraitSnapshotService`
- `TraitGenerationService`

## DAL
- `TraitSnapshotRepository`
- `TraitGenerationRunRepository`

## Views
- Trait profile panel
- Snapshot history view
