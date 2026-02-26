# Entity Resolution Module Design

## Purpose
Resolve duplicate or ambiguous sources, topics, and entities across ingestion pipelines.

## Responsibilities
- Match/merge near-duplicate entities.
- Keep merge history and reversible decisions.
- Provide canonical IDs for downstream analytics.

## Schema (Proposed)
- `entity_candidate` (`id`, `entity_type`, `raw_key`, `attributes_json`, `created_at`)
- `entity_canonical` (`id`, `entity_type`, `canonical_key`, `attributes_json`, `updated_at`)
- `entity_merge_log` (`id`, `canonical_id`, `merged_candidate_id`, `reason`, `created_at`)

## Handlers (HTTP/API)
- `POST /internal/entities/resolve`
- `GET /api/admin/entities/pending`
- `POST /api/admin/entities/merge`

## Services
- `EntityResolutionService`
- `EntityMergeService`

## DAL
- `EntityCandidateRepository`
- `EntityCanonicalRepository`
- `EntityMergeLogRepository`

## Views
- Duplicate resolution queue (admin)
