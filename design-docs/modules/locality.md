# Locality Module Design

## Purpose
Model Philippine locality hierarchy and provide lookup/query APIs for geospatial and analytics use.

## Responsibilities
- Maintain locality hierarchy from barangay to region.
- Support hierarchy traversal and locality search.
- Provide locality metadata for trend/simulation modules.

## Schema (Proposed)
- `locality` (`id`, `code`, `name`, `type`, `parent_id`, `created_at`, `updated_at`)
- `locality_alias` (`id`, `locality_id`, `alias`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/localities`
- `GET /api/localities/:id`
- `GET /api/localities/:id/children`
- `GET /api/localities/search`

## Services
- `LocalityService`
- `LocalityTreeService`

## DAL
- `LocalityRepository`
- `LocalityAliasRepository`

## Views
- Locality selectors
- Hierarchy breadcrumbs
