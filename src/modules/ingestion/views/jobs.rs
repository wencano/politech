use crate::ui::layout::app_shell::page_shell;

/// Render the ingest jobs special page — shows job queue status and history.
/// Stub — wire to GET /admin/ingest/jobs once a protected HTML route is added.
#[allow(dead_code)]
pub fn render() -> String {
    page_shell(
        "Ingest Jobs",
        "Job queue and execution history",
        r#"
        <div class="sf-card">
          <h2 class="sf-card-title">Ingest Jobs</h2>
          <p class="mt-2 text-sm text-sf-muted">
            Use the JSON API at <code>/api/admin/ingest/jobs</code> to monitor job history.
            Trigger jobs via <code>POST /internal/ingest/run/{source_id}</code>.
          </p>
        </div>
        "#,
    )
}
