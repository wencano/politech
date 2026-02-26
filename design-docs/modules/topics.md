# Topics Module Design

## Purpose
Maintain canonical topic taxonomy and mapping for trend and recommendation features.

## Responsibilities
- Store topics, aliases, and category hierarchy.
- Resolve mentions to canonical topics.
- Support topic search and filtering.
- Serve topic analytics for stats, sentiment, and **alignment map** metrics (topic trait vector vs locality population → 1–10 dislike–meh–like per locality).

## Schema (Proposed)
- `topic` (`id`, `name`, `slug`, `parent_id`, `status`, `created_at`)
- `topic_alias` (`id`, `topic_id`, `alias`, `language`, `created_at`)
- `topic_taxonomy` (`id`, `topic_id`, `category`, `tags_json`, `updated_at`)
- `topic_stats_daily` (`id`, `topic_id`, `bucket_date`, `mention_count`, `source_count`, `growth_rate`, `confidence_score`)
- `topic_sentiment_daily` (`id`, `topic_id`, `bucket_date`, `positive_count`, `neutral_count`, `negative_count`)
- `alignment_map_metric` (shared with persona/policy: `alignable_type=topic`, `alignable_id=topic_id`, `alignment_1_10`, etc.); optional `topic_map_metric_daily` for intensity-only.

## Handlers (HTTP/API)
- `GET /api/topics`
- `GET /api/topics/:id`
- `GET /api/topics/:id/stats`
- `GET /api/topics/:id/sentiment`
- `GET /api/topics/:id/map`
- `POST /api/topics`
- `PATCH /api/topics/:id`

## Services
- `TopicService`
- `TopicResolutionService`
- `TopicStatsService`
- `TopicSentimentService`
- `TopicMapMetricsService`

## DAL
- `TopicRepository`
- `TopicAliasRepository`
- `TopicTaxonomyRepository`
- `TopicStatsDailyRepository`
- `TopicSentimentDailyRepository`
- `TopicMapMetricDailyRepository`

## Views
- Topic explorer
- Topic detail page
- Topic stats panel
- Sentiment trend chart
- Topic alignment map (1–10 dislike–meh–like by locality, from trait alignment)
