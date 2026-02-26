# Settings Module Design

## Purpose
Manage user and workspace-level settings used by dashboard, simulations, and chats.

## Responsibilities
- Persist UI preferences and default filters.
- Manage notification and privacy settings.
- Expose typed defaults for downstream modules.

## Schema (Proposed)
- `user_setting` (`user_id`, `key`, `value_json`, `updated_at`)
- `workspace_setting` (`workspace_id`, `key`, `value_json`, `updated_at`)

## Handlers (HTTP/API)
- `GET /api/settings`
- `PATCH /api/settings`

## Services
- `SettingsService`
- `SettingsValidationService`

## DAL
- `UserSettingRepository`
- `WorkspaceSettingRepository`

## Views
- Preferences page
- Notification settings
