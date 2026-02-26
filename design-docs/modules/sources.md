# Sources Module Design

## Purpose
Manage upstream content sources used for trend extraction and scoring.

## Responsibilities
- Register media, influencer, think tank, and party sources.
- Track source trust and coverage metadata.
- Provide source config references to ingestion jobs.

## Schema (Proposed)
- `source` (`id`, `name`, `type`, `base_url`, `status`, `trust_score`, `created_at`, `updated_at`)
- `source_coverage` (`id`, `source_id`, `domain`, `locality_scope`, `language`, `updated_at`)

## Handlers (HTTP/API)
- `GET /api/sources`
- `POST /api/sources`
- `PATCH /api/sources/:id`
- `POST /api/sources/:id/disable`

## Services
- `SourceService`
- `SourceTrustService`

## DAL
- `SourceRepository`
- `SourceCoverageRepository`

## Views
- Source catalog table
- Source quality panel
