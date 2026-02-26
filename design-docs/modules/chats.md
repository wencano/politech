# Chats Module Design

## Purpose
Manage chat threads and conversation-level metadata. Provide the container that groups chat messages and connects user interactions to orchestration runs.

## Responsibilities
- Create, read, update, archive, and soft-delete chat threads.
- Enforce ownership and visibility permissions.
- Store chat-level context and defaults (provider/model preferences).

## Boundaries
- Owns chat thread metadata and lifecycle.
- Does not store individual message bodies (Chat Messages module owns that).
- Does not call providers directly (Orchestrator owns execution).

## Schema (Proposed)

### `chat`
- `id` (uuid, pk)
- `user_id` (uuid, fk user.id)
- `title` (text)
- `status` (enum: active, archived, deleted)
- `visibility` (enum: private, org, public)
- `preferred_provider` (text, nullable)
- `preferred_model` (text, nullable)
- `context_kind` (enum: general, simulation, recommendation)
- `context_ref_id` (uuid, nullable)
- `last_message_at` (timestamptz, nullable)
- `created_at`, `updated_at` (timestamptz)
- `deleted_at` (timestamptz, nullable)

### `chat_participant` (optional for future shared chats)
- `chat_id` (uuid, fk chat.id)
- `user_id` (uuid, fk user.id)
- `role` (enum: owner, editor, viewer)
- `joined_at` (timestamptz)
- Primary key: (`chat_id`, `user_id`)

## Handlers (HTTP/API)
- `POST /api/chats`
  - Create chat with optional context reference.
- `GET /api/chats/:id`
  - Get chat metadata and summary.
- `GET /api/chats`
  - List user chats (cursor pagination).
- `PATCH /api/chats/:id`
  - Update title, visibility, preferred provider/model.
- `POST /api/chats/:id/archive`
  - Archive thread.
- `DELETE /api/chats/:id`
  - Soft-delete thread.

## Services
- `ChatService`
  - Thread lifecycle, ownership checks, validation.
- `ChatAccessService`
  - Authorization and visibility policy evaluation.
- `ChatSummaryService`
  - Lightweight chat list projections (message count, latest snippet pointers).

## DAL (Data Access Layer)
- `ChatRepository`
  - CRUD + status transitions + cursor-based listing.
- `ChatParticipantRepository`
  - Participant management for shared thread scenarios.
- `ChatProjectionRepository`
  - Read-optimized list and dashboard projections.

## Views (UI)
- Chat sidebar list: title, last activity, status.
- Chat header: context badge (simulation/recommendation/general), provider/model preference.
- Chat management actions: rename, archive, delete, share visibility.

## Module Events
- Emits:
  - `chat.created`
  - `chat.updated`
  - `chat.archived`
  - `chat.deleted`
- Consumes:
  - `chat.message_created` (to update `last_message_at`)
  - `orchestration.run_succeeded` (for chat-level usage summaries)
