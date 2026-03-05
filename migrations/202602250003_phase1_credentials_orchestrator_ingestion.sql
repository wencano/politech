-- Phase 1: Credentials, Orchestrator skeleton, Ingestion baseline, Abuse actions

-- ============================================================
-- CREDENTIALS MODULE
-- ============================================================

CREATE TABLE IF NOT EXISTS credential (
    id                  uuid PRIMARY KEY,
    provider            text NOT NULL,
    label               text NOT NULL,
    status              text NOT NULL DEFAULT 'active',
    encrypted_secret    text NOT NULL,
    secret_version      int  NOT NULL DEFAULT 1,
    capabilities        jsonb NOT NULL DEFAULT '{}',
    priority_weight     int  NOT NULL DEFAULT 100,
    created_by_user_id  uuid NOT NULL REFERENCES app_user(id),
    created_at          timestamptz NOT NULL DEFAULT now(),
    updated_at          timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_credential_status_provider ON credential(status, provider);

CREATE TABLE IF NOT EXISTS credential_session (
    id                      uuid PRIMARY KEY,
    credential_id           uuid NOT NULL REFERENCES credential(id) ON DELETE CASCADE,
    provider_session_id     text,
    access_token_encrypted  text,
    refresh_token_encrypted text,
    expires_at              timestamptz,
    last_refreshed_at       timestamptz,
    updated_at              timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_credential_session_cred ON credential_session(credential_id);

CREATE TABLE IF NOT EXISTS credential_health (
    credential_id           uuid PRIMARY KEY REFERENCES credential(id) ON DELETE CASCADE,
    last_success_at         timestamptz,
    last_failure_at         timestamptz,
    failure_count_window    int NOT NULL DEFAULT 0,
    cooldown_until          timestamptz,
    retry_after             timestamptz,
    recent_latency_ms_p95   int,
    recent_error_rate       numeric
);

-- ============================================================
-- ORCHESTRATOR MODULE (skeleton)
-- ============================================================

CREATE TABLE IF NOT EXISTS orchestration_run (
    id                       uuid PRIMARY KEY,
    request_type             text NOT NULL,
    chat_id                  uuid,
    request_idempotency_key  text UNIQUE,
    provider                 text NOT NULL,
    credential_id            uuid REFERENCES credential(id),
    status                   text NOT NULL DEFAULT 'queued',
    attempt_count            int  NOT NULL DEFAULT 0,
    started_at               timestamptz,
    ended_at                 timestamptz,
    error_code               text,
    error_message            text,
    created_at               timestamptz NOT NULL DEFAULT now(),
    updated_at               timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_orchestration_run_status ON orchestration_run(status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_orchestration_run_cred   ON orchestration_run(credential_id, created_at DESC);

CREATE TABLE IF NOT EXISTS orchestration_chunk (
    id               bigserial PRIMARY KEY,
    run_id           uuid NOT NULL REFERENCES orchestration_run(id) ON DELETE CASCADE,
    seq_no           int  NOT NULL,
    delta_text       text NOT NULL,
    provider_payload jsonb,
    created_at       timestamptz NOT NULL DEFAULT now(),
    UNIQUE (run_id, seq_no)
);

CREATE TABLE IF NOT EXISTS orchestration_usage (
    run_id              uuid PRIMARY KEY REFERENCES orchestration_run(id) ON DELETE CASCADE,
    input_tokens        int     NOT NULL DEFAULT 0,
    output_tokens       int     NOT NULL DEFAULT 0,
    estimated_cost_usd  numeric NOT NULL DEFAULT 0,
    latency_ms          int,
    updated_at          timestamptz NOT NULL DEFAULT now()
);

-- ============================================================
-- INGESTION MODULE (baseline)
-- ============================================================

CREATE TABLE IF NOT EXISTS source_config (
    id                      uuid PRIMARY KEY,
    name                    text NOT NULL,
    ingest_mode             text NOT NULL DEFAULT 'api',
    base_url                text NOT NULL,
    poll_interval_seconds   int  NOT NULL DEFAULT 3600,
    status                  text NOT NULL DEFAULT 'active',
    coverage_meta           jsonb NOT NULL DEFAULT '{}',
    compliance_flags        jsonb NOT NULL DEFAULT '{}',
    created_at              timestamptz NOT NULL DEFAULT now(),
    updated_at              timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_source_config_status ON source_config(status);

CREATE TABLE IF NOT EXISTS ingest_job (
    id            uuid PRIMARY KEY,
    source_id     uuid NOT NULL REFERENCES source_config(id),
    mode          text NOT NULL,
    status        text NOT NULL DEFAULT 'pending',
    started_at    timestamptz,
    ended_at      timestamptz,
    error_message text,
    created_at    timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ingest_job_source ON ingest_job(source_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ingest_job_status  ON ingest_job(status);

CREATE TABLE IF NOT EXISTS raw_ingest_event (
    id            uuid PRIMARY KEY,
    source_id     uuid NOT NULL REFERENCES source_config(id),
    ingest_job_id uuid REFERENCES ingest_job(id),
    payload_json  jsonb NOT NULL,
    checksum      text  NOT NULL,
    fetched_at    timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_raw_ingest_event_source ON raw_ingest_event(source_id, fetched_at DESC);
CREATE INDEX IF NOT EXISTS idx_raw_ingest_event_chksum ON raw_ingest_event(checksum);

-- ============================================================
-- ABUSE / RISK MODULE (baseline)
-- ============================================================

CREATE TABLE IF NOT EXISTS abuse_action (
    id            uuid PRIMARY KEY,
    subject_type  text NOT NULL,
    subject_key   text NOT NULL,
    action        text NOT NULL,
    expires_at    timestamptz,
    reason        text,
    created_at    timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_abuse_action_subject ON abuse_action(subject_type, subject_key, expires_at);
