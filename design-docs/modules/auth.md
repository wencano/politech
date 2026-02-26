# Auth Module Design

## Purpose
Authenticate users, issue sessions, and enforce secure access for protected endpoints.

## Responsibilities
- Email/password and OAuth login flows.
- Session issuance, refresh, and revocation.
- Password reset and account verification.

## Schema (Proposed)
- `auth_identity` (`id`, `user_id`, `provider`, `provider_user_id`, `created_at`)
- `password_credential` (`user_id`, `password_hash`, `password_updated_at`)
- `auth_challenge` (`id`, `user_id`, `challenge_type`, `status`, `expires_at`)

## Handlers (HTTP/API)
- `POST /api/auth/register`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `POST /api/auth/refresh`
- `POST /api/auth/password/forgot`
- `POST /api/auth/password/reset`

## Services
- `AuthService`
- `PasswordService`
- `SessionTokenService`
- `AuthChallengeService`

## DAL
- `AuthIdentityRepository`
- `PasswordCredentialRepository`
- `AuthChallengeRepository`

## Views
- Sign in
- Sign up
- Password reset
