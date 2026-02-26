use crate::ui::layout::app_shell::page_shell;

/// Render the ingestion sources list page.
/// Stub — wire to GET /sources once a protected HTML route is added.
#[allow(dead_code)]
pub fn render() -> String {
    page_shell(
        "Ingestion Sources",
        "Configured data sources for ingestion",
        r#"
        <div class="sf-card">
          <h2 class="sf-card-title">Sources</h2>
          <p class="mt-2 text-sm text-sf-muted">
            Use the JSON API at <code>/api/sources</code> to list and register ingestion sources.
          </p>
        </div>
        "#,
    )
}
