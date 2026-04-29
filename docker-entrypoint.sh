#!/bin/sh
set -eu

if [ -z "${APP_SECRET_KEY:-}" ]; then
  echo "ERROR: APP_SECRET_KEY is not set" >&2
  exit 1
fi

# Dokploy/env files often add trailing whitespace or quotes
SECRET=$(printf '%s' "$APP_SECRET_KEY" | tr -d ' \t\r\n"')
SECRET_LEN=${#SECRET}
if [ "$SECRET_LEN" -ne 64 ]; then
  echo "ERROR: APP_SECRET_KEY must be exactly 64 hex chars (got ${SECRET_LEN}). Run: openssl rand -hex 32" >&2
  exit 1
fi

case "${DATABASE_URL:-}" in
  *localhost*|*127.0.0.1*)
    echo "ERROR: DATABASE_URL must use host 'postgres' inside Docker, not localhost" >&2
    exit 1
    ;;
  "")
    echo "ERROR: DATABASE_URL is not set" >&2
    exit 1
    ;;
esac

case "${REDIS_URL:-}" in
  *127.0.0.1*|*localhost*)
    echo "ERROR: REDIS_URL must use host 'redis' inside Docker, not localhost" >&2
    exit 1
    ;;
esac

echo "politech: starting (DATABASE_URL host should be postgres, REDIS_URL host redis)"
exec /app/politech
