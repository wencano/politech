use crate::ui::layout::app_shell::page_shell;

/// Render the credentials list page.
/// Stub — wire to GET /credentials once a protected HTML route is added.
#[allow(dead_code)]
pub fn render() -> String {
    page_shell(
        "Credentials",
        "Manage provider API credentials",
        r#"
        <div class="sf-card">
          <h2 class="sf-card-title">Credentials</h2>
          <p class="mt-2 text-sm text-sf-muted">
            Use the JSON API at <code>/api/credentials</code> to list, create, and manage credentials.
          </p>
        </div>
        "#,
    )
}
