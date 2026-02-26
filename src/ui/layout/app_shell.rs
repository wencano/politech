use leptos::prelude::*;

// ── Leptos component (for future full Leptos SSR integration) ─────────────────

#[component]
pub fn AppShell(
    title: &'static str,
    subtitle: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <link rel="stylesheet" href="/public/styles.css" />
            </head>
            <body class="bg-sf-base text-sf-text">
                <div class="min-h-screen">
                    <header class="border-b border-sf-border bg-sf-surface">
                        <div class="mx-auto max-w-7xl px-6 py-4">
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-xs uppercase tracking-wide text-sf-muted">Politech</p>
                                    <h1 class="text-xl font-semibold">{title}</h1>
                                </div>
                                <nav class="flex gap-3 text-sm">
                                    <a class="sf-nav-link" href="/">Home</a>
                                    <a class="sf-nav-link" href="/dashboard">Dashboard</a>
                                    <a class="sf-nav-link" href="/health">Health</a>
                                </nav>
                            </div>
                            <p class="mt-2 text-sm text-sf-muted">{subtitle}</p>
                        </div>
                    </header>
                    <main class="mx-auto max-w-7xl p-6">{children()}</main>
                </div>
            </body>
        </html>
    }
}

// ── Raw HTML helpers (used by page handlers until full Leptos SSR is wired) ───

/// Wrap `body_html` in the standard page shell.
pub fn page_shell(title: &str, subtitle: &str, body_html: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>{title}</title>
  <link rel="stylesheet" href="/public/styles.css"/>
</head>
<body class="bg-sf-base text-sf-text">
  <div class="min-h-screen">
    <header class="border-b border-sf-border bg-sf-surface">
      <div class="mx-auto max-w-7xl px-6 py-4">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-xs uppercase tracking-wide text-sf-muted">Politech</p>
            <h1 class="text-xl font-semibold">{title}</h1>
          </div>
          <nav class="flex gap-3 text-sm">
            <a class="sf-nav-link" href="/">Home</a>
            <a class="sf-nav-link" href="/dashboard">Dashboard</a>
            <a class="sf-nav-link" href="/health">Health</a>
          </nav>
        </div>
        <p class="mt-2 text-sm text-sf-muted">{subtitle}</p>
      </div>
    </header>
    <main class="mx-auto max-w-7xl p-6">
      {body_html}
    </main>
  </div>
</body>
</html>"#
    )
}
