# Simulations Module Design

## Purpose
Define and execute policy/persona simulation requests.

## Responsibilities
- Persist simulation definitions and parameters.
- Queue simulation execution.
- Track status and ownership.

## Schema (Proposed)
- `simulation` (`id`, `user_id`, `name`, `input_json`, `scope_json`, `status`, `created_at`, `updated_at`)
- `simulation_run` (`id`, `simulation_id`, `run_no`, `status`, `started_at`, `ended_at`)

## Handlers (HTTP/API)
- `POST /api/simulations`
- `GET /api/simulations/:id`
- `POST /api/simulations/:id/run`
- `GET /api/simulations/:id/runs`

## Services
- `SimulationService`
- `SimulationExecutionService`

## DAL
- `SimulationRepository`
- `SimulationRunRepository`

## Views
- Simulation builder form
- Simulation status timeline
