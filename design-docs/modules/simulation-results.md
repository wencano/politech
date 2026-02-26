# Simulation Results Module Design

## Purpose
Store result snapshots from simulation runs and expose comparison-ready outputs.

## Responsibilities
- Persist immutable simulation result snapshots.
- Provide comparison views across runs.
- Link results to recommendation requests.

## Schema (Proposed)
- `simulation_result` (`id`, `simulation_run_id`, `score_json`, `locality_breakdown_json`, `created_at`)
- `simulation_result_metric` (`id`, `result_id`, `metric_key`, `metric_value`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/simulations/:id/results`
- `GET /api/simulation-results/:resultId`
- `POST /api/simulation-results/compare`

## Services
- `SimulationResultService`
- `ResultComparisonService`

## DAL
- `SimulationResultRepository`
- `SimulationResultMetricRepository`

## Views
- Result heatmap/table
- Multi-run comparison view
