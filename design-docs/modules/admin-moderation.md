# Admin and Moderation Module Design

## Purpose
Provide privileged operations for managing data quality, abuse incidents, and policy controls.

## Responsibilities
- Admin controls for sources, groups, topics, and credentials.
- Moderation of flagged content and user actions.
- Role-based access checks for sensitive operations.

## Schema (Proposed)
- `admin_action` (`id`, `admin_user_id`, `action`, `target_type`, `target_id`, `metadata_json`, `created_at`)
- `moderation_case` (`id`, `subject_type`, `subject_id`, `status`, `reason`, `opened_at`, `closed_at`)

## Handlers (HTTP/API)
- `GET /api/admin/*`
- `POST /api/admin/moderation/cases`
- `PATCH /api/admin/moderation/cases/:id`

## Services
- `AdminActionService`
- `ModerationService`

## DAL
- `AdminActionRepository`
- `ModerationCaseRepository`

## Views
- Admin console
- Moderation case queue
