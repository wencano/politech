# Ingestion and Enrichment Module Design

## Purpose
Ingest source content via API/scraping and enrich into normalized topic mentions.

## Responsibilities
- Schedule and execute source fetch jobs.
- Normalize payloads and extract topics/entities.
- Emit trend facts and update aggregates.

## Schema (Proposed)
- `ingest_job` (`id`, `source_id`, `mode`, `status`, `started_at`, `ended_at`)
- `raw_ingest_event` (`id`, `source_id`, `payload_json`, `checksum`, `fetched_at`)
- `topic_mention` (`id`, `source_id`, `topic_id`, `text`, `sentiment`, `locality_id`, `created_at`)

## Handlers (HTTP/API)
- `POST /internal/ingest/run/:sourceId`
- `POST /internal/ingest/replay/:jobId`
- `GET /api/admin/ingest/jobs`

## Services
- `IngestionService`
- `EnrichmentService`
- `DeduplicationService`

## DAL
- `IngestJobRepository`
- `RawIngestEventRepository`
- `TopicMentionRepository`

## Views
- Ingestion job monitor (admin)
- Source payload diagnostics (admin)
