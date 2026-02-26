# Orchestrator Module Design

## Purpose
Coordinate provider interactions for chat and other AI workloads. Route requests to the next available credential, ensure session/token readiness, send prompt payloads, and stream responses reliably.

## Responsibilities
- Select the best available credential for each request.
- Create or refresh provider session token when needed.
- Adapt prompt payloads to provider-specific request format.
- Stream provider responses to clients while persisting chunks.
- Retry transient failures and fail over to alternate credentials.

## Boundaries
- Owns orchestration state and provider call flow.
- Delegates credential storage/health writes to Credentials module.
- Delegates message persistence to Chats/Chat Messages modules.

## Schema (Proposed)

### `orchestration_run`
- `id` (uuid, pk)
- `request_type` (enum: chat, recommendation, simulation_aux)
- `chat_id` (uuid, nullable)
- `request_idempotency_key` (text, unique)
- `provider` (text)
- `credential_id` (uuid, fk credential.id)
- `status` (enum: queued, streaming, completed, failed, cancelled)
- `attempt_count` (int)
- `started_at`, `ended_at` (timestamptz, nullable)
- `error_code`, `error_message` (text, nullable)
- `created_at`, `updated_at` (timestamptz)

### `orchestration_chunk`
- `id` (bigserial, pk)
- `run_id` (uuid, fk orchestration_run.id)
- `seq_no` (int)
- `delta_text` (text)
- `provider_payload` (jsonb, nullable)
- `created_at` (timestamptz)

### `orchestration_usage`
- `run_id` (uuid, pk/fk orchestration_run.id)
- `input_tokens` (int)
- `output_tokens` (int)
- `estimated_cost_usd` (numeric)
- `latency_ms` (int)
- `updated_at` (timestamptz)

## Handlers (HTTP/API)
- `POST /api/chats/:id/messages` (entrypoint integration)
  - Triggers orchestration run for assistant response.
- `GET /api/chats/:id/stream`
  - Streams run chunks to client over SSE/WebSocket.
- `POST /api/orchestrator/runs/:id/cancel`
  - Cancels active stream when user aborts.
- Internal:
  - `POST /internal/orchestrator/retry/:id`
  - `POST /internal/orchestrator/health-callback`

## Services
- `OrchestratorService`
  - Main execution state machine.
  - Handles retries and failover policy.
- `ProviderAdapterService`
  - Provider abstraction for send/stream/refresh operations.
- `SessionTokenService`
  - Refreshes and validates provider session tokens.
- `StreamingRelayService`
  - Fans out chunks to client and persistence targets.

## DAL (Data Access Layer)
- `OrchestrationRunRepository`
  - Create run, update state transitions with optimistic concurrency.
- `OrchestrationChunkRepository`
  - Append ordered chunks and support idempotent chunk insertion.
- `OrchestrationUsageRepository`
  - Upsert final token/cost/latency metrics.

## Views (UI/Operator)
- Admin run monitor (status, provider, credential, retries, latency).
- Per-chat stream diagnostics (chunk rate, disconnects, completion status).
- Incident panel for failed runs and rate-limit bursts.

## Routing Policy (Default)
- Candidate filter:
  - Matching provider + required capability + `status=active`.
  - Not in cooldown / retry_after window.
- Scoring:
  - Weighted round-robin by `priority_weight`.
  - Penalize high error rate and high p95 latency.
- Failover:
  - Retry once on same credential for transient timeout.
  - Then route to next candidate credential.

## Module Events
- Emits:
  - `orchestration.run_started`
  - `orchestration.chunk_emitted`
  - `orchestration.run_succeeded`
  - `orchestration.run_failed`
  - `orchestration.credential_rate_limited`
- Consumes:
  - `chat.message_created` (user role)
  - `credential.health_changed`
