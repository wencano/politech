# Abuse Detection and Risk Module Design

## Purpose
Detect abusive traffic, assign risk scores, and enforce adaptive protection policies.

## Responsibilities
- Compute request risk from IP/account/device/behavior signals.
- Trigger challenge, throttle, tarpitting, or block actions.
- Publish abuse insights to edge and application layers.

## Schema (Proposed)
- `risk_signal` (`id`, `signal_type`, `subject_type`, `subject_key`, `value`, `captured_at`)
- `risk_score` (`id`, `subject_type`, `subject_key`, `score`, `level`, `reason_json`, `updated_at`)
- `abuse_action` (`id`, `subject_type`, `subject_key`, `action`, `expires_at`, `created_at`)

## Handlers (HTTP/API)
- `POST /internal/risk/evaluate`
- `POST /internal/risk/feedback`
- `GET /api/admin/abuse/actions`

## Services
- `RiskScoringService`
- `AbuseActionService`
- `ChallengeDecisionService`

## DAL
- `RiskSignalRepository`
- `RiskScoreRepository`
- `AbuseActionRepository`

## Views
- Abuse events dashboard
- Block/challenge management console
