# Dashboard Module Design

## Purpose
Provide the primary authenticated experience for trends, simulations, and recommendations.

## Responsibilities
- Aggregate data from trends, groups, and locality modules.
- Serve cached dashboard widgets and map overlays.
- Manage dashboard-level filters and saved views.
- Expose topic analytics widgets (stats, sentiment, and map layers).

## Schema (Proposed)
- `dashboard_view` (`id`, `user_id`, `name`, `filter_json`, `is_default`, `created_at`)
- `dashboard_widget_state` (`id`, `view_id`, `widget_key`, `config_json`, `updated_at`)
- `dashboard_topic_widget_cache` (`id`, `user_id`, `topic_id`, `cache_key`, `payload_json`, `expires_at`)

## Handlers (HTTP/API)
- `GET /api/dashboard`
- `GET /api/dashboard/widgets`
- `GET /api/dashboard/topics/:id/overview`
- `POST /api/dashboard/views`
- `PATCH /api/dashboard/views/:id`

## Services
- `DashboardService`
- `DashboardProjectionService`
- `DashboardCacheService`
- `TopicAnalyticsWidgetService`

## DAL
- `DashboardViewRepository`
- `DashboardWidgetStateRepository`
- `DashboardTopicWidgetCacheRepository`

## Views
- Main dashboard shell
- Trend cards
- Simulation summary cards
- Map insight panel
- Topic analytics panel (stats + sentiment + map)
