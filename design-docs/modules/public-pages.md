# Public Pages Module Design

## Purpose
Serve marketing and informational pages for unauthenticated users, and drive account conversion.

## Responsibilities
- Render landing, product, pricing, FAQ, and legal pages.
- Provide SEO metadata and sitemap generation.
- Gate high-cost features behind sign-in prompts.

## Schema (Proposed)
- `page_content` (`id`, `slug`, `title`, `body_md`, `status`, `published_at`, `updated_at`)
- `conversion_event` (`id`, `session_fingerprint`, `event_type`, `referrer`, `created_at`)

## Handlers (HTTP/API)
- `GET /`
- `GET /about`
- `GET /docs`
- `GET /pricing`
- `GET /legal/*`

## Services
- `PublicPageService` (content retrieval and rendering)
- `SeoService` (meta tags, sitemap, robots policy)

## DAL
- `PageContentRepository`
- `ConversionEventRepository`

## Views
- Landing page
- Feature overview
- Sign-up call-to-action blocks
