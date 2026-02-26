# Notification Module Design

## Purpose
Deliver user notifications for completed simulations, recommendations, and system events.

## Responsibilities
- Persist in-app notification records.
- Dispatch email/push/webhook channels (as enabled).
- Support read/unread and preference filtering.

## Schema (Proposed)
- `notification` (`id`, `user_id`, `type`, `title`, `body`, `status`, `created_at`, `read_at`)
- `notification_delivery` (`id`, `notification_id`, `channel`, `status`, `attempt_count`, `last_attempt_at`)

## Handlers (HTTP/API)
- `GET /api/notifications`
- `POST /api/notifications/:id/read`
- `POST /internal/notifications/dispatch/:id`

## Services
- `NotificationService`
- `NotificationDispatchService`

## DAL
- `NotificationRepository`
- `NotificationDeliveryRepository`

## Views
- Notification center
- Toast alerts
