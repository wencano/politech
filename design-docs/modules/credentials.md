# Credentials Module Design

## Purpose
Manage upstream provider credentials used by the orchestrator. Provide secure storage, health tracking, rotation metadata, and availability decisions for credential routing.

## Responsibilities
- Store and manage provider credentials and related session/token metadata.
- Expose safe read models for admin/operator usage (never expose raw secret values).
- Track credential health and capacity signals used by routing logic.
- Support lifecycle transitions and cooldown/rate-limit handling.

## Boundaries
- Owns credential records and health state.
- Does not send prompts directly to providers (orchestrator owns that).
- Does not perform business chat logic beyond credential selection support.

## Schema (Proposed)

### `credential`
- `id` (uuid, pk)
- `provider` (text; e.g., openai, anthropic, custom)
- `label` (text)
- `status` (enum: active, cooldown, rate_limited, disabled, expired)
- `encrypted_secret` (bytea/text)
- `secret_version` (int)
- `capabilities` (jsonb; supported models/modes)
- `priority_weight` (int; for weighted balancing)
- `created_by_user_id` (uuid, fk user.id)
- `created_at`, `updated_at` (timestamptz)

### `credential_session`
- `id` (uuid, pk)
- `credential_id` (uuid, fk credential.id)
- `provider_session_id` (text, nullable)
- `access_token_encrypted` (bytea/text, nullable)
- `refresh_token_encrypted` (bytea/text, nullable)
- `expires_at` (timestamptz, nullable)
- `last_refreshed_at` (timestamptz, nullable)
- `updated_at` (timestamptz)

### `credential_health`
- `credential_id` (uuid, pk/fk credential.id)
- `last_success_at` (timestamptz, nullable)
- `last_failure_at` (timestamptz, nullable)
- `failure_count_window` (int)
- `cooldown_until` (timestamptz, nullable)
- `retry_after` (timestamptz, nullable)
- `recent_latency_ms_p95` (int, nullable)
- `recent_error_rate` (numeric, nullable)

## Handlers (HTTP/API)
- `GET /api/credentials`
  - Returns sanitized list (id, provider, label, status, health summary).
- `POST /api/credentials`
  - Creates a new credential with encrypted secret.
- `PATCH /api/credentials/:id`
  - Updates status, label, weight, or rotates secret.
- `POST /api/credentials/:id/disable`
  - Fast disable for incidents.
- `POST /api/credentials/:id/rotate`
  - Rotates secret and bumps secret version.

## Services
- `CredentialService`
  - CRUD + lifecycle transitions.
  - Secret encryption/decryption orchestration using key management abstraction.
- `CredentialHealthService`
  - Updates health from orchestrator callbacks (success, failure, rate-limit signals).
  - Applies automatic cooldown and reactivation rules.
- `CredentialSelectionService`
  - Returns next available credential candidates by provider + policy.

## DAL (Data Access Layer)
- `CredentialRepository`
  - Filter by provider/status/capability.
  - Upsert encrypted secret versions.
- `CredentialSessionRepository`
  - Read/write provider session tokens and expiry metadata.
- `CredentialHealthRepository`
  - Atomic health updates for concurrent orchestration runs.

## Views (UI/Operator)
- Credential list with status badge, provider, and health indicators.
- Credential detail with capability tags and recent incident timeline.
- Rotation and disable actions with audit logging.

## Security and Compliance
- Secrets encrypted at rest with envelope encryption.
- Redact sensitive fields in logs and traces.
- Require privileged role for write/rotation actions.
- Maintain audit trail for create/update/disable/rotate events.

## Module Events
- Emits:
  - `credential.created`
  - `credential.updated`
  - `credential.disabled`
  - `credential.rotated`
  - `credential.health_changed`
- Consumes:
  - `orchestration.run_succeeded`
  - `orchestration.run_failed`
  - `orchestration.credential_rate_limited`
