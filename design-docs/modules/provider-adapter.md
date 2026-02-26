# Provider Adapter Module Design

## Purpose
Standardize integration with multiple AI/chat providers behind a single interface for orchestrator use.

## Responsibilities
- Normalize request/response payloads across providers.
- Support streaming, retries, and token/session refresh hooks.
- Surface provider capability metadata to routing logic.

## Schema (Proposed)
- `provider_capability` (`id`, `provider`, `model`, `features_json`, `updated_at`)
- `provider_error_map` (`id`, `provider`, `error_code`, `normalized_code`, `retryable`, `updated_at`)

## Handlers (HTTP/API)
- Internal only; no public API.
- `POST /internal/provider-adapter/test-connection`

## Services
- `ProviderAdapterRegistry`
- `ProviderRequestMapperService`
- `ProviderResponseMapperService`

## DAL
- `ProviderCapabilityRepository`
- `ProviderErrorMapRepository`

## Views
- Provider capability matrix (admin/internal)
