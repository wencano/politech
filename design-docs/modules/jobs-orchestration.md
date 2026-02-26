# Jobs Orchestration Module Design

## Purpose
Schedule and coordinate asynchronous background jobs across ingestion, simulations, extrapolation, and maintenance tasks.

## Responsibilities
- Queue management and worker dispatch.
- Retry/backoff policies with dead-letter handling.
- Idempotency and deduplication for recurring jobs.

## Schema (Proposed)
- `job_queue` (`id`, `job_type`, `payload_json`, `priority`, `status`, `scheduled_at`, `started_at`, `ended_at`)
- `job_attempt` (`id`, `job_id`, `attempt_no`, `status`, `error_message`, `created_at`)
- `job_dead_letter` (`id`, `job_id`, `reason`, `payload_json`, `created_at`)

## Handlers (HTTP/API)
- `POST /internal/jobs/enqueue`
- `POST /internal/jobs/retry/:id`
- `GET /api/admin/jobs`
- `GET /api/admin/jobs/dead-letter`

## Services
- `JobSchedulerService`
- `JobDispatchService`
- `JobRetryService`

## DAL
- `JobQueueRepository`
- `JobAttemptRepository`
- `JobDeadLetterRepository`

## Views
- Job queue monitor
- Failed jobs and retry controls
