# Search and Discovery Module Design

## Purpose
Provide full-text and semantic search across topics, sources, simulations, and chats.

## Responsibilities
- Index searchable entities with ranking metadata.
- Support keyword and vector similarity queries.
- Expose faceted search for locality/group filters.

## Schema (Proposed)
- `search_document` (`id`, `doc_type`, `doc_id`, `title`, `body_text`, `metadata_json`, `updated_at`)
- `search_embedding` (`doc_id`, `embedding`, `updated_at`)

## Handlers (HTTP/API)
- `GET /api/search`
- `POST /internal/search/reindex`

## Services
- `SearchService`
- `SemanticSearchService`
- `SearchIndexService`

## DAL
- `SearchDocumentRepository`
- `SearchEmbeddingRepository`

## Views
- Global search box
- Search results with facets
