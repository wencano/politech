//! Leptos WASM client — Salesforce-style two-column layout:
//! left panel (topic input + history), right panel (Leaflet map).
#![cfg(feature = "client")]

use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Function, Reflect};
use leptos::mount;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::window;

#[unsafe(no_mangle)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(|| view! { <App /> });
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicItem {
    pub id: String,
    pub title: String,
    pub created_at: String,
    #[serde(default)]
    pub traits: Option<serde_json::Value>,
}

/// HEXACO order for radar chart and trait display (must match backend).
const HEXACO_ORDER: &[&str] = &[
    "honesty_humility",
    "emotionality",
    "extraversion",
    "agreeableness",
    "conscientiousness",
    "openness",
];

fn hexaco_label(key: &str) -> String {
    match key {
        "honesty_humility" => "Honesty-Humility".to_string(),
        "emotionality" => "Emotionality".to_string(),
        "extraversion" => "Extraversion".to_string(),
        "agreeableness" => "Agreeableness".to_string(),
        "conscientiousness" => "Conscientiousness".to_string(),
        "openness" => "Openness".to_string(),
        _ => key.replace('_', " "),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlignmentItem {
    pub code: String,
    pub geojson_key: Option<String>,
    pub name: String,
    #[serde(rename = "alignment_01")]
    pub alignment_01: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    ok: bool,
    data: T,
}

fn init_map() {
    let _ = window().and_then(|w| {
        let pm = Reflect::get(&w, &JsValue::from_str("PolitechMap")).ok()?;
        let init_fn = Reflect::get(&pm, &JsValue::from_str("init"))
            .ok()?
            .dyn_ref::<Function>()?
            .clone();
        let _ = init_fn.call2(
            &JsValue::NULL,
            &JsValue::from_str("map-container"),
            &JsValue::from_str("/public/geojson/country.0.001.json"),
        );
        Some(())
    });
}

fn set_map_alignment(alignments: &[AlignmentItem]) {
    let Ok(arr) = JsValue::from_serde(alignments) else {
        return;
    };
    let _ = window().and_then(|w| {
        let pm = Reflect::get(&w, &JsValue::from_str("PolitechMap")).ok()?;
        let set_fn = Reflect::get(&pm, &JsValue::from_str("setAlignment"))
            .ok()?
            .dyn_ref::<Function>()?
            .clone();
        let _ = set_fn.call1(&pm, &arr);
        Some(())
    });
}

fn clear_map_alignment() {
    set_map_alignment(&[]);
}

fn draw_detail_charts(
    radar_labels: Vec<String>,
    radar_values: Vec<f64>,
    bar_labels: Vec<String>,
    bar_values: Vec<f64>,
) {
    let _ = window().and_then(|w| {
        let charts = Reflect::get(&w, &JsValue::from_str("PolitechCharts")).ok()?;
        let radar_fn = Reflect::get(&charts, &JsValue::from_str("radar")).ok()?.dyn_ref::<Function>()?.clone();
        let bar_fn = Reflect::get(&charts, &JsValue::from_str("regionalBar")).ok()?.dyn_ref::<Function>()?.clone();
        let radar_opts = serde_json::json!({ "labels": radar_labels, "values": radar_values });
        let bar_opts = serde_json::json!({ "labels": bar_labels, "values": bar_values });
        let radar_opts_js = JsValue::from_serde(&radar_opts).ok()?;
        let bar_opts_js = JsValue::from_serde(&bar_opts).ok()?;
        let _ = radar_fn.call2(&charts, &JsValue::from_str("topic-radar-canvas"), &radar_opts_js);
        let _ = bar_fn.call2(&charts, &JsValue::from_str("topic-regional-bar-canvas"), &bar_opts_js);
        Some(())
    });
}

fn destroy_detail_charts() {
    let _ = window().and_then(|w| {
        let charts = Reflect::get(&w, &JsValue::from_str("PolitechCharts")).ok()?;
        let destroy_fn = Reflect::get(&charts, &JsValue::from_str("destroy")).ok()?.dyn_ref::<Function>()?.clone();
        let _ = destroy_fn.call1(&charts, &JsValue::from_str("topic-radar-canvas"));
        let _ = destroy_fn.call1(&charts, &JsValue::from_str("topic-regional-bar-canvas"));
        Some(())
    });
}

fn invalidate_map_size() {
    let _ = window().and_then(|w| {
        let pm = Reflect::get(&w, &JsValue::from_str("PolitechMap")).ok()?;
        let fn_ = Reflect::get(&pm, &JsValue::from_str("invalidateSize"))
            .ok()?
            .dyn_ref::<Function>()?
            .clone();
        let _ = fn_.call0(&pm);
        Some(())
    });
}

fn set_topic_route(topic_id: &str, edit: bool) {
    let suffix = if edit { "?edit=1" } else { "" };
    let _ = window().and_then(|w| {
        let history = w.history().ok()?;
        let _ = history.replace_state_with_url(
            &JsValue::NULL,
            "",
            Some(&format!("/app/topic/{topic_id}{suffix}")),
        );
        Some(())
    });
}

fn navigate_app_root() {
    let _ = window().and_then(|w| {
        let history = w.history().ok()?;
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some("/app/"));
        Some(())
    });
}

fn get_topic_id_from_url() -> Option<String> {
    let w = window()?;
    let loc = w.location();

    if let Ok(pathname) = loc.pathname() {
        let parts: Vec<&str> = pathname.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 && parts[0] == "app" && parts[1] == "topic" {
            return Some(parts[2].to_string());
        }
    }

    if let Ok(search) = loc.search() {
        let query = search.trim_start_matches('?');
        for pair in query.split('&') {
            let mut kv = pair.splitn(2, '=');
            if kv.next() == Some("topic") {
                if let Some(v) = kv.next() {
                    if !v.is_empty() {
                        return Some(v.to_string());
                    }
                }
            }
        }
    }
    None
}

fn is_desktop() -> bool {
    let Some(w) = window() else {
        return false;
    };
    let Ok(js_w) = w.inner_width() else {
        return false;
    };
    js_w.as_f64().map(|w| w >= 1025.0).unwrap_or(false)
}

fn get_edit_mode_from_url() -> bool {
    let Some(w) = window() else {
        return false;
    };
    let Ok(search) = w.location().search() else {
        return false;
    };
    let query = search.trim_start_matches('?');
    query
        .split('&')
        .any(|pair| pair == "edit=1" || pair.starts_with("edit=1&"))
}

#[component]
fn App() -> impl IntoView {
    let (topics, set_topics) = signal::<Vec<TopicItem>>(Vec::new());
    let (selected_id, set_selected_id) = signal::<Option<String>>(None);
    let (alignment_data, set_alignment_data) = signal::<Vec<AlignmentItem>>(Vec::new());
    let (editing_topic_id, set_editing_topic_id) = signal::<Option<String>>(None);
    let (open_menu_for, set_open_menu_for) = signal::<Option<String>>(None);
    let (message, set_message) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    let (sidebar_open, set_sidebar_open) = signal(true);
    let (details_modal_open, set_details_modal_open) = signal(false);
    let (detail_tab, set_detail_tab) = signal("overview".to_string());

    let load_alignment = move |topic_id: String| {
        spawn_local(async move {
            match gloo_net::http::Request::get(&format!("/api/map/alignment/{topic_id}"))
                .send()
                .await
            {
                Ok(resp) if resp.ok() => {
                    if let Ok(api) = serde_json::from_str::<ApiResponse<Vec<AlignmentItem>>>(
                        &resp.text().await.unwrap_or_default(),
                    ) {
                        set_map_alignment(&api.data);
                        set_alignment_data.set(api.data);
                    }
                }
                _ => set_error.set(Some("Failed to load map alignment".into())),
            }
        });
    };

    let load_topics = move || {
        let selected_from_url = get_topic_id_from_url();
        let edit_mode_from_url = get_edit_mode_from_url();
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            match gloo_net::http::Request::get("/api/topics").send().await {
                Ok(resp) if resp.ok() => {
                    if let Ok(api) = serde_json::from_str::<ApiResponse<Vec<TopicItem>>>(
                        &resp.text().await.unwrap_or_default(),
                    ) {
                        let loaded = api.data;
                        set_topics.set(loaded.clone());
                        if let Some(topic_id) = selected_from_url {
                            set_selected_id.set(Some(topic_id.clone()));
                            set_details_modal_open.set(is_desktop());
                            load_alignment(topic_id.clone());
                            if edit_mode_from_url {
                                if let Some(topic) = loaded.iter().find(|t| t.id == topic_id) {
                                    set_message.set(topic.title.clone());
                                    set_editing_topic_id.set(Some(topic.id.clone()));
                                }
                            }
                        }
                    }
                }
                Ok(_) => set_error.set(Some("Failed to load topics".into())),
                Err(e) => set_error.set(Some(format!("Error: {e}"))),
            }
            set_loading.set(false);
        });
    };

    let on_submit = move |_| {
        let msg = message.get().trim().to_string();
        if msg.is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        set_open_menu_for.set(None);

        spawn_local(async move {
            let body = serde_json::json!({ "title": msg });
            if let Some(edit_id) = editing_topic_id.get_untracked() {
                match gloo_net::http::Request::patch(&format!("/api/topics/{edit_id}"))
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        if let Ok(api) = serde_json::from_str::<ApiResponse<TopicItem>>(
                            &resp.text().await.unwrap_or_default(),
                        ) {
                            set_topics.update(|items| {
                                if let Some(idx) = items.iter().position(|t| t.id == api.data.id) {
                                    items[idx] = api.data.clone();
                                }
                            });
                            set_selected_id.set(Some(api.data.id.clone()));
                            set_editing_topic_id.set(None);
                            set_message.set(String::new());
                            set_topic_route(&api.data.id, false);
                            load_alignment(api.data.id);
                        }
                    }
                    Ok(_) => set_error.set(Some("Failed to update topic".into())),
                    Err(e) => set_error.set(Some(format!("Error: {e}"))),
                }
            } else {
                match gloo_net::http::Request::post("/api/topics")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        if let Ok(api) = serde_json::from_str::<ApiResponse<TopicItem>>(
                            &resp.text().await.unwrap_or_default(),
                        ) {
                            set_topics.update(|items| items.insert(0, api.data.clone()));
                            set_selected_id.set(Some(api.data.id.clone()));
                            set_message.set(String::new());
                            set_topic_route(&api.data.id, false);
                            load_alignment(api.data.id);
                        }
                    }
                    Ok(_) => set_error.set(Some("Failed to create topic".into())),
                    Err(e) => set_error.set(Some(format!("Error: {e}"))),
                }
            }
            set_loading.set(false);
        });
    };

    let on_select_topic = move |id: String| {
        set_editing_topic_id.set(None);
        set_open_menu_for.set(None);
        set_selected_id.set(Some(id.clone()));
        set_details_modal_open.set(is_desktop());
        set_topic_route(&id, false);
        load_alignment(id);
    };

    let on_edit_topic = move |id: String| {
        set_open_menu_for.set(None);
        if let Some(topic) = topics.get_untracked().into_iter().find(|t| t.id == id) {
            set_message.set(topic.title);
            set_selected_id.set(Some(topic.id.clone()));
            set_editing_topic_id.set(Some(topic.id.clone()));
            set_topic_route(&topic.id, true);
        }
    };

    let on_delete_topic = move |id: String| {
        set_open_menu_for.set(None);
        set_loading.set(true);
        set_error.set(None);
        spawn_local(async move {
            match gloo_net::http::Request::delete(&format!("/api/topics/{id}"))
                .send()
                .await
            {
                Ok(resp) if resp.ok() => {
                    set_topics.update(|items| items.retain(|t| t.id != id));
                    if selected_id.get_untracked().as_deref() == Some(id.as_str()) {
                        set_selected_id.set(None);
                        set_editing_topic_id.set(None);
                        set_alignment_data.set(Vec::new());
                        set_message.set(String::new());
                    }
                }
                Ok(_) => set_error.set(Some("Failed to delete topic".into())),
                Err(e) => set_error.set(Some(format!("Error: {e}"))),
            }
            set_loading.set(false);
        });
    };

    let on_close_topic_panel = move |_| {
        set_selected_id.set(None);
        set_alignment_data.set(Vec::new());
        set_details_modal_open.set(false);
        clear_map_alignment();
        navigate_app_root();
    };

    Effect::new(move |_| {
        if selected_id.get().is_none() {
            clear_map_alignment();
        }
    });

    Effect::new(move |_| {
        let _ = details_modal_open.get();
        let _ = window().and_then(|w| {
            let cb = Closure::wrap(Box::new(move || invalidate_map_size()) as Box<dyn FnMut()>);
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                150,
            );
            cb.forget();
            Some(())
        });
    });

    Effect::new(move |_| {
        let open = details_modal_open.get();
        if !open {
            destroy_detail_charts();
            return;
        }
        let id = match selected_id.get() {
            Some(id) => id,
            None => return,
        };
        let topics_list = topics.get();
        let topic = topics_list.into_iter().find(|t| t.id == id);
        let Some(t) = topic else { return };
        let alignments = alignment_data.get();
        if alignments.is_empty() {
            return;
        }
        let radar_labels: Vec<String> = HEXACO_ORDER.iter().map(|k| hexaco_label(k)).collect();
        let radar_values: Vec<f64> = t
            .traits
            .as_ref()
            .and_then(|v| v.as_object())
            .map(|obj| {
                HEXACO_ORDER
                    .iter()
                    .filter_map(|k| obj.get(*k).and_then(|v| v.as_f64()))
                    .collect()
            })
            .unwrap_or_default();
        let bar_labels: Vec<String> = alignments.iter().map(|a| a.name.clone()).collect();
        let bar_values: Vec<f64> = alignments.iter().map(|a| a.alignment_01).collect();
        let _ = window().and_then(|w| {
            let rl = radar_labels.clone();
            let rv = radar_values.clone();
            let bl = bar_labels.clone();
            let bv = bar_values.clone();
            let cb = Closure::once(move || {
                draw_detail_charts(rl, rv, bl, bv);
            });
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                80,
            );
            cb.forget();
            Some(())
        });
    });

    Effect::new(move |_| {
        load_topics();
        let _ = window().and_then(|w| {
            let cb = Closure::wrap(Box::new(move || init_map()) as Box<dyn FnMut()>);
            let _ =
                w.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 120);
            cb.forget();
            Some(())
        });
    });

    view! {
        <div
            class=move || {
                if sidebar_open.get() {
                    "app-shell sidebar-open"
                } else {
                    "app-shell sidebar-closed"
                }
            }
        >
            <button
                class="app-sidebar-toggle"
                aria-label="Toggle sidebar"
                title="Toggle sidebar"
                on:click=move |_| set_sidebar_open.update(|v| *v = !*v)
            >
                <span class="app-sidebar-toggle-icon" aria-hidden="true">"≡"</span>
            </button>
            <aside class="topics-panel">
                <header class="topics-header">
                    <div class="topics-header-row">
                        <h1 class="topics-title">"Topic Resonance Map"</h1>
                        <button
                            class="sidebar-close-btn"
                            aria-label="Close sidebar"
                            title="Close sidebar"
                            on:click=move |_| set_sidebar_open.set(false)
                        >
                            "×"
                        </button>
                    </div>
                    <p class="topics-subtitle">
                        "Submit a message/topic. Traits are generated and used for region alignment."
                    </p>
                </header>

                <section class="topics-form">
                    {move || editing_topic_id.get().map(|_| view! {
                        <p class="topic-history-title">"Editing topic (?edit=1 active)"</p>
                    })}
                    <textarea
                        class="topic-input"
                        rows=4
                        placeholder="e.g. Infrastructure and roads"
                        prop:value=move || message.get()
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                    />
                    <button class="topic-submit" on:click=on_submit disabled=loading>
                        {move || {
                            if loading.get() {
                                "Submitting...".to_string()
                            } else if editing_topic_id.get().is_some() {
                                "Save changes".to_string()
                            } else {
                                "Submit topic".to_string()
                            }
                        }}
                    </button>
                </section>

                {move || error.get().map(|e| view! { <p class="topic-error">{e}</p> })}

                <section class="topic-history">
                    <p class="topic-history-title">"History (click to view heat map)"</p>
                    <ul class="topic-list">
                        {move || topics.get().into_iter().map(|t| {
                            let item_id = t.id.clone();
                            let click_id = t.id.clone();
                            let menu_id = t.id.clone();
                            let menu_toggle_id = t.id.clone();
                            let edit_id = t.id.clone();
                            let delete_id = t.id.clone();
                            let selected = move || selected_id.get().as_ref() == Some(&item_id);
                            view! {
                                <li>
                                    <div class="topic-row">
                                        <button
                                            class=move || {
                                                if selected() {
                                                    "topic-item topic-item-active".to_string()
                                                } else {
                                                    "topic-item".to_string()
                                                }
                                            }
                                            on:click=move |_| on_select_topic(click_id.clone())
                                        >
                                            {t.title}
                                        </button>
                                        <div class="topic-menu-wrap">
                                            <button
                                                class="topic-menu-trigger"
                                                on:click=move |_| {
                                                    if open_menu_for.get().as_ref() == Some(&menu_toggle_id) {
                                                        set_open_menu_for.set(None);
                                                    } else {
                                                        set_open_menu_for.set(Some(menu_toggle_id.clone()));
                                                    }
                                                }
                                            >
                                                "..."
                                            </button>
                                            {move || {
                                                if open_menu_for.get().as_ref() == Some(&menu_id) {
                                                    let edit_click_id = edit_id.clone();
                                                    let delete_click_id = delete_id.clone();
                                                    view! {
                                                        <div class="topic-menu">
                                                            <button class="topic-menu-item" on:click=move |_| on_edit_topic(edit_click_id.clone())>
                                                                "Edit"
                                                            </button>
                                                            <button class="topic-menu-item topic-menu-item-danger" on:click=move |_| on_delete_topic(delete_click_id.clone())>
                                                                "Delete"
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <></> }.into_any()
                                                }
                                            }}
                                        </div>
                                    </div>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </section>
            </aside>

            {move || sidebar_open.get().then(|| view! {
                <div
                    class="sidebar-overlay"
                    aria-hidden="true"
                    on:click=move |_| set_sidebar_open.set(false)
                ></div>
            })}
            <main
                class=move || {
                    let open = details_modal_open.get() && selected_id.get().is_some();
                    if open {
                        "map-panel details-open"
                    } else {
                        "map-panel"
                    }
                }
            >
                <div class="map-area">
                    <div id="map-container"></div>
                </div>
                {move || {
                    let has_selection = selected_id.get().is_some();
                    if has_selection {
                        Some(view! {
                            <button
                                class="app-fab"
                                aria-label="Toggle topic details"
                                title="Topic details"
                                on:click=move |_| set_details_modal_open.update(|v| *v = !*v)
                            >
                                {move || if details_modal_open.get() { "✕" } else { "ℹ" }}
                            </button>
                        })
                    } else {
                        None
                    }
                }}
                {move || {
                    if !details_modal_open.get() {
                        return None;
                    }
                    let id = match selected_id.get() {
                        Some(id) => id,
                        None => return None,
                    };
                    let topic = topics.get().into_iter().find(|t| t.id == id);
                    topic.map(|t| {
                        let alignments = alignment_data.get();
                        let traits_view = t.traits.as_ref().and_then(|v| v.as_object()).map(|obj| {
                            let pairs: Vec<_> = HEXACO_ORDER
                                .iter()
                                .filter_map(|&k| {
                                    let val = obj.get(k)?;
                                    let s = match val {
                                        serde_json::Value::Number(n) => {
                                            n.as_f64().map(|f| format!("{:.2}", f)).unwrap_or_else(|| val.to_string())
                                        }
                                        serde_json::Value::String(s) => s.clone(),
                                        _ => val.to_string(),
                                    };
                                    Some((hexaco_label(k), s))
                                })
                                .collect();
                            pairs
                        });
                        view! {
                            <div class="topic-detail-panel">
                                <div class="topic-detail-header">
                                    <h2 class="topic-detail-title">{t.title.clone()}</h2>
                                    <button class="topic-detail-close" on:click=on_close_topic_panel title="Close / Back to app">
                                        "×"
                                    </button>
                                </div>
                                <div class="topic-detail-meta">
                                    <span class="topic-detail-id">"ID: " {t.id.clone()}</span>
                                    <span class="topic-detail-date">{t.created_at.clone()}</span>
                                </div>
                                <div class="topic-detail-tabs">
                                    <button
                                        class=move || if detail_tab.get() == "overview" { "topic-detail-tab topic-detail-tab-active" } else { "topic-detail-tab" }
                                        on:click=move |_| set_detail_tab.set("overview".to_string())
                                    >
                                        "Overview"
                                    </button>
                                    <button
                                        class=move || if detail_tab.get() == "details" { "topic-detail-tab topic-detail-tab-active" } else { "topic-detail-tab" }
                                        on:click=move |_| set_detail_tab.set("details".to_string())
                                    >
                                        "Details"
                                    </button>
                                </div>
                                <div class="topic-detail-tab-content">
                                    <div class=move || if detail_tab.get() == "overview" { "topic-detail-tab-pane topic-detail-tab-pane-active" } else { "topic-detail-tab-pane" }>
                                        <p class="topic-detail-chart-label">"National Traits"</p>
                                        <div class="topic-detail-chart-wrap">
                                            <canvas id="topic-radar-canvas" width="280" height="220"></canvas>
                                        </div>
                                        <p class="topic-detail-chart-label">"Regional alignment (%)"</p>
                                        <div class="topic-detail-chart-wrap">
                                            <canvas id="topic-regional-bar-canvas" width="280" height="320"></canvas>
                                        </div>
                                        <section class="topic-detail-section">
                                            <h3 class="topic-detail-section-title">"Regional alignment"</h3>
                                            <div class="topic-detail-alignment-wrap">
                                                <table class="topic-detail-alignment">
                                                    <thead>
                                                        <tr><th>"Region"</th><th>"Alignment"</th></tr>
                                                    </thead>
                                                    <tbody>
                                                        {alignments.iter().map(|a| view! {
                                                            <tr>
                                                                <td>{a.name.clone()}</td>
                                                                <td class="align-val">{format!("{:.0}%", a.alignment_01 * 100.0)}</td>
                                                            </tr>
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </section>
                                    </div>
                                    <div class=move || if detail_tab.get() == "details" { "topic-detail-tab-pane topic-detail-tab-pane-active" } else { "topic-detail-tab-pane" }>
                                        {traits_view.map(|pairs| view! {
                                            <section class="topic-detail-section">
                                                <h3 class="topic-detail-section-title">"Traits (0–1)"</h3>
                                                <ul class="topic-detail-traits">
                                                    {pairs.into_iter().map(|(k, v)| view! {
                                                        <li class="topic-detail-trait"><span class="trait-key">{k}</span> " → " <span class="trait-val">{v}</span></li>
                                                    }).collect_view()}
                                                </ul>
                                            </section>
                                        }.into_any()).unwrap_or_else(|| view! { <p class="topic-detail-no-traits">"No traits."</p> }.into_any())}
                                    </div>
                                </div>
                                <div class="topic-detail-footer">
                                    <a href="/app/" class="topic-detail-back" on:click=move |ev| {
                                        ev.prevent_default();
                                        on_close_topic_panel(ev);
                                    }>"Back to /app/"</a>
                                </div>
                            </div>
                        }
                    })
                }}
            </main>
        </div>
    }
}
