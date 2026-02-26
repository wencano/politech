# Feature Flags and Experiments Module Design

## Purpose
Control feature rollout and run controlled experiments safely.

## Responsibilities
- Define feature flags and targeting rules.
- Assign experiment variants deterministically.
- Track exposure and outcome metrics.

## Schema (Proposed)
- `feature_flag` (`id`, `key`, `status`, `rule_json`, `updated_at`)
- `experiment` (`id`, `key`, `status`, `variant_json`, `started_at`, `ended_at`)
- `experiment_exposure` (`id`, `experiment_id`, `user_id`, `variant`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/flags`
- `POST /api/admin/flags`
- `POST /api/admin/experiments`

## Services
- `FeatureFlagService`
- `ExperimentService`

## DAL
- `FeatureFlagRepository`
- `ExperimentRepository`
- `ExperimentExposureRepository`

## Views
- Admin flag manager
- Experiment dashboard
