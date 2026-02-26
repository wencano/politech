# API and SDK Boundary Module Design

## Purpose
Define stable API contracts and internal client boundaries for web and future external consumers.

## Responsibilities
- Maintain versioned API schemas.
- Provide typed SDK wrappers for internal modules/clients.
- Enforce backward compatibility policy.

## Schema (Proposed)
- `api_contract` (`id`, `name`, `version`, `schema_json`, `status`, `created_at`)
- `api_deprecation_notice` (`id`, `contract_id`, `notice`, `effective_at`, `created_at`)

## Handlers (HTTP/API)
- `GET /api/meta/contracts`
- `GET /api/meta/contracts/:name/:version`

## Services
- `ApiContractService`
- `CompatibilityService`
- `SdkGenerationService`

## DAL
- `ApiContractRepository`
- `ApiDeprecationNoticeRepository`

## Views
- API contract catalog (internal)
