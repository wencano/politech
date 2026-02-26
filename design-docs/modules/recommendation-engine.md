# Recommendation Engine Module Design

## Purpose
Generate actionable recommendations to improve policy/persona alignment.

## Responsibilities
- Evaluate simulation outputs and topic/group context.
- Produce ranked recommendation items with rationale.
- Persist requests and feedback for quality iteration.

## Schema (Proposed)
- `recommendation_request` (`id`, `user_id`, `basis_type`, `basis_id`, `status`, `created_at`)
- `recommendation_item` (`id`, `request_id`, `rank_no`, `suggestion_text`, `impact_score`, `rationale_json`, `created_at`)
- `recommendation_feedback` (`id`, `request_id`, `user_id`, `rating`, `notes`, `created_at`)

## Handlers (HTTP/API)
- `POST /api/recommendations`
- `GET /api/recommendations/:id`
- `POST /api/recommendations/:id/feedback`

## Services
- `RecommendationService`
- `RecommendationRankingService`
- `RecommendationFeedbackService`

## DAL
- `RecommendationRequestRepository`
- `RecommendationItemRepository`
- `RecommendationFeedbackRepository`

## Views
- Recommendation results panel
- Recommendation feedback UI
