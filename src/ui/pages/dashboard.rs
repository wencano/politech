use axum::response::{Html, IntoResponse};

use crate::ui::layout::app_shell::page_shell;

pub async fn dashboard() -> impl IntoResponse {
    Html(page_shell(
        "Dashboard",
        "Salesforce CRM-inspired shell — Phase 1 Foundation",
        r#"
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
          <section class="sf-card">
            <h2 class="sf-card-title">Auth Status</h2>
            <p class="mt-2 text-sm text-sf-muted">
              Use /api/auth/* endpoints for register, login, logout, refresh.
            </p>
          </section>
          <section class="sf-card">
            <h2 class="sf-card-title">Locality Data</h2>
            <p class="mt-2 text-sm text-sf-muted">
              Use /api/localities for hierarchy drill-down integration.
            </p>
          </section>
          <section class="sf-card">
            <h2 class="sf-card-title">Next — Phase 2</h2>
            <p class="mt-2 text-sm text-sf-muted">
              Trend ingestion, groups, alignment maps, and simulations.
            </p>
          </section>
        </div>
        "#,
    ))
}
