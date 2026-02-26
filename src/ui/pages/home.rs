use axum::response::{Html, IntoResponse};

use crate::ui::layout::app_shell::page_shell;

pub async fn home() -> impl IntoResponse {
    Html(page_shell(
        "Politech Phase 1",
        "Leptos + Tailwind + Salesforce CRM-inspired UI baseline",
        r#"
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
          <section class="sf-card">
            <h2 class="sf-card-title">Foundation Ready</h2>
            <ul class="mt-2 list-disc pl-5 text-sm text-sf-muted">
              <li>Auth and sessions API baseline</li>
              <li>Locality hierarchy API baseline</li>
              <li>Dashboard shell and design tokens</li>
              <li>Credentials module (AES-256-GCM encrypted secrets)</li>
              <li>Orchestrator skeleton (health-aware credential routing)</li>
              <li>Ingestion baseline (source config + job queue)</li>
            </ul>
          </section>
          <section class="sf-card">
            <h2 class="sf-card-title">Phase 1 APIs</h2>
            <ul class="mt-2 list-disc pl-5 text-sm text-sf-muted">
              <li>POST /api/auth/register, login, logout, refresh</li>
              <li>GET /api/localities</li>
              <li>GET|POST /api/credentials</li>
              <li>GET /api/orchestrator/runs</li>
              <li>GET|POST /api/sources</li>
            </ul>
          </section>
        </div>
        "#,
    ))
}
