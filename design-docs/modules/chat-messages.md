# Chat Messages Module Design

## Purpose
Store ordered messages and streaming deltas for chats. Ensure reliable persistence, idempotency, and replay for streamed assistant output.

## Responsibilities
- Persist message records by chat and role.
- Store stream chunks and finalize assistant messages.
- Track provider metadata, token usage, and message provenance.
- Support read paths for history loading and stream replay.

## Boundaries
- Owns message body data and chunk ordering semantics.
- Depends on Chats for chat ownership and lifecycle checks.
- Receives generated assistant output via Orchestrator.

## Schema (Proposed)

### `chat_message`
- `id` (uuid, pk)
- `chat_id` (uuid, fk chat.id)
- `role` (enum: system, user, assistant, tool)
- `status` (enum: pending, streaming, completed, failed, cancelled)
- `content_text` (text, nullable until finalized)
- `content_json` (jsonb, nullable for structured/tool payloads)
- `provider` (text, nullable)
- `model` (text, nullable)
- `orchestration_run_id` (uuid, nullable fk orchestration_run.id)
- `idempotency_key` (text, nullable unique per chat)
- `token_input` (int, nullable)
- `token_output` (int, nullable)
- `cost_usd` (numeric, nullable)
- `created_at`, `updated_at` (timestamptz)

### `chat_message_chunk`
- `id` (bigserial, pk)
- `message_id` (uuid, fk chat_message.id)
- `seq_no` (int)
- `delta_text` (text)
- `created_at` (timestamptz)
- Unique key: (`message_id`, `seq_no`)

### `chat_message_attachment` (optional)
- `id` (uuid, pk)
- `message_id` (uuid, fk chat_message.id)
- `kind` (enum: image, file, link, citation)
- `payload` (jsonb)
- `created_at` (timestamptz)

## Handlers (HTTP/API)
- `POST /api/chats/:id/messages`
  - Create user message and trigger orchestrator.
- `GET /api/chats/:id/messages`
  - Paginated message history.
- `GET /api/chats/:id/messages/:messageId/chunks`
  - Replay stream chunks for reconnect/recovery.
- Internal:
  - `POST /internal/messages/:id/chunks`
  - `POST /internal/messages/:id/finalize`

## Services
- `MessageService`
  - Create user messages, finalize assistant messages, update statuses.
- `MessageChunkService`
  - Append chunks idempotently and reconstruct stream snapshots.
- `MessageUsageService`
  - Attach usage/cost metadata from orchestration outcomes.

## DAL (Data Access Layer)
- `ChatMessageRepository`
  - Insert/update/list messages with cursor pagination.
- `ChatMessageChunkRepository`
  - Append and read chunk streams with sequence guarantees.
- `ChatMessageProjectionRepository`
  - Fast read model for chat transcript rendering.

## Views (UI)
- Transcript panel with role-based rendering and streaming cursor.
- Retry/regenerate actions for failed assistant messages.
- Message metadata drawer (provider, model, tokens, cost, timestamps).

## Consistency and Idempotency
- Use `idempotency_key` for client retry safety on user message creation.
- Use (`message_id`, `seq_no`) unique constraint for chunk deduplication.
- Mark message `completed` only after finalize event persists.

## Module Events
- Emits:
  - `chat.message_created`
  - `chat.message_streaming`
  - `chat.message_completed`
  - `chat.message_failed`
- Consumes:
  - `orchestration.chunk_emitted`
  - `orchestration.run_succeeded`
  - `orchestration.run_failed`
