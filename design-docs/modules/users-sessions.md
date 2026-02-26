# Users and Sessions Module Design

## Purpose
Store user profiles and active session state with audit-friendly lifecycle management.

## Responsibilities
- User account storage and profile linkage.
- Session tracking by device/browser fingerprint.
- Session revocation and audit trails.

## Schema (Proposed)
- `user` (`id`, `email`, `status`, `created_at`, `updated_at`)
- `user_session` (`id`, `user_id`, `device_hash`, `ip_hash`, `status`, `expires_at`, `created_at`, `updated_at`)
- `session_audit` (`id`, `session_id`, `event`, `metadata`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/users/me`
- `GET /api/users/me/sessions`
- `POST /api/users/me/sessions/:id/revoke`
- `POST /api/users/me/sessions/revoke-all`

## Services
- `UserService`
- `SessionService`
- `SessionAuditService`

## DAL
- `UserRepository`
- `UserSessionRepository`
- `SessionAuditRepository`

## Views
- Account overview
- Active sessions list
- Security controls
