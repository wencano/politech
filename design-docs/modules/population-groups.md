# Population Groups Module Design

## Purpose
Manage seeded and dynamically-created political population groups used in trend analysis.

## Responsibilities
- Store group definitions and membership criteria.
- Promote dynamic groups when threshold rules are met.
- Track lifecycle states and merge/deprecate history.

## Schema (Proposed)
- `population_group` (`id`, `name`, `slug`, `origin`, `status`, `created_at`, `updated_at`)
- `population_group_rule` (`id`, `group_id`, `rule_json`, `threshold`, `updated_at`)
- `population_group_event` (`id`, `group_id`, `event_type`, `metadata`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/groups`
- `POST /api/groups`
- `PATCH /api/groups/:id`
- `POST /api/groups/evaluate-dynamic`

## Services
- `PopulationGroupService`
- `DynamicGroupEvaluatorService`

## DAL
- `PopulationGroupRepository`
- `PopulationGroupRuleRepository`
- `PopulationGroupEventRepository`

## Views
- Group list and trend ranking
- Group detail and rule inspector
