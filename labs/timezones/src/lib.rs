use chrono::{Offset, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use dioxus::prelude::*;
use ui::LabMeta;
use ui::searchable_dropdown::SearchableDropdown;

pub const META: LabMeta = LabMeta {
    slug: "tz",
    name: "Timezones",
    description: "Compare timezones side-by-side — add multiple timezones, click to snap to an hour, and share the link.",
    tags: &["tool", "time", "calendar"],
};

#[cfg(not(target_arch = "wasm32"))]
mod browser {
    pub fn get_search_params() -> String {
        String::new()
    }
    pub fn set_search_params(_params: &str) {}
    pub fn detect_local_tz() -> Option<String> {
        None
    }
    pub fn prefers_12h() -> bool { false }
    pub fn get_picker_origin() -> Option<(f64, f64)> { None }
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use wasm_bindgen::prelude::*;

    pub fn get_search_params() -> String {
        web_sys::window()
            .and_then(|w| w.location().search().ok())
            .unwrap_or_default()
            .trim_start_matches('?')
            .to_string()
    }

    pub fn set_search_params(params: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                let new_url = if params.is_empty() {
                    window.location().pathname().ok().unwrap_or_default()
                } else {
                    format!(
                        "{}?{}",
                        window.location().pathname().ok().unwrap_or_default(),
                        params
                    )
                };
                let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&new_url));
            }
        }
    }

    pub fn detect_local_tz() -> Option<String> {
        let formatter = js_sys::Intl::DateTimeFormat::new(
            &js_sys::Array::new(),
            &js_sys::Object::new(),
        );
        let options = formatter.resolved_options();
        js_sys::Reflect::get(&options, &JsValue::from("timeZone"))
            .ok()
            .and_then(|v| v.as_string())
            .filter(|s| !s.is_empty())
    }

    pub fn prefers_12h() -> bool {
        let opts = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&opts, &JsValue::from("hour"), &JsValue::from("numeric"));
        let _ = js_sys::Reflect::set(&opts, &JsValue::from("minute"), &JsValue::from("numeric"));
        let formatter = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &opts);
        formatter
            .format()
            .call1(&JsValue::undefined(), &js_sys::Date::new_0())
            .ok()
            .and_then(|v| v.as_string())
            .is_some_and(|s| s.contains("AM") || s.contains("PM") || s.contains("am") || s.contains("pm"))
    }

    pub fn get_picker_origin() -> Option<(f64, f64)> {
        let window = web_sys::window()?;
        let doc = window.document()?;
        let el = doc.get_element_by_id("add-tz-btn")?;
        let rect = el.get_bounding_client_rect();
        Some((rect.left(), rect.top() + rect.height() + 4.0))
    }
}

const COMMON_TIMEZONES: &[&str] = &[
    "Pacific/Pago_Pago",
    "Pacific/Honolulu",
    "America/Anchorage",
    "America/Los_Angeles",
    "America/Phoenix",
    "America/Denver",
    "America/Chicago",
    "America/New_York",
    "America/Halifax",
    "America/St_Johns",
    "America/Sao_Paulo",
    "America/Argentina/Buenos_Aires",
    "America/Nuuk",
    "Atlantic/Azores",
    "UTC",
    "Europe/London",
    "Europe/Berlin",
    "Europe/Paris",
    "Europe/Madrid",
    "Europe/Rome",
    "Europe/Amsterdam",
    "Europe/Stockholm",
    "Europe/Prague",
    "Europe/Warsaw",
    "Europe/Budapest",
    "Europe/Vienna",
    "Europe/Copenhagen",
    "Europe/Oslo",
    "Europe/Zurich",
    "Europe/Belgrade",
    "Europe/Sofia",
    "Europe/Bucharest",
    "Europe/Athens",
    "Europe/Helsinki",
    "Europe/Kyiv",
    "Europe/Minsk",
    "Europe/Moscow",
    "Europe/Istanbul",
    "Asia/Dubai",
    "Asia/Tehran",
    "Asia/Kabul",
    "Asia/Karachi",
    "Asia/Kolkata",
    "Asia/Kathmandu",
    "Asia/Dhaka",
    "Asia/Bangkok",
    "Asia/Jakarta",
    "Asia/Singapore",
    "Asia/Shanghai",
    "Asia/Tokyo",
    "Asia/Seoul",
    "Australia/Eucla",
    "Australia/Adelaide",
    "Australia/Sydney",
    "Australia/Lord_Howe",
    "Pacific/Norfolk",
    "Pacific/Auckland",
    "Pacific/Chatham",
    "Pacific/Kiritimati",
];

fn filter_timezones(query: &str) -> Vec<&'static str> {
    if query.is_empty() {
        return COMMON_TIMEZONES.to_vec();
    }
    let q = query.to_lowercase();
    COMMON_TIMEZONES
        .iter()
        .copied()
        .filter(|tz| tz.to_lowercase().contains(&q))
        .collect()
}

fn parse_tz(name: &str) -> Option<Tz> {
    name.parse::<Tz>().ok()
}

fn format_offset(tz: Tz) -> String {
    let now = Utc::now();
    let offset = tz.offset_from_utc_datetime(&now.naive_utc());
    let secs = offset.fix().local_minus_utc();
    if secs == 0 {
        return "+00:00".to_string();
    }
    let hours = secs / 3600;
    let mins = (secs.abs() / 60) % 60;
    let sign = if secs >= 0 { '+' } else { '-' };
    format!("{}{:02}:{:02}", sign, hours.abs(), mins)
}

fn local_time_for_utc_hour(tz: Tz, utc_hour: u32) -> (u32, i32) {
    let now = Utc::now();
    let today = now.date_naive();
    let Some(naive_dt) = today.and_hms_opt(utc_hour, 0, 0) else {
        return (0, 0);
    };
    let utc_dt = Utc.from_utc_datetime(&naive_dt);
    let local = utc_dt.with_timezone(&tz);
    let local_hour = local.hour();
    let day_diff = local.date_naive() - today;
    (local_hour, day_diff.num_days() as i32)
}

fn current_utc_hour() -> u32 {
    Utc::now().hour()
}

fn parse_search_params(search: &str) -> (Vec<String>, Option<u32>) {
    let mut timezones = Vec::new();
    let mut selected = None;

    for pair in search.split('&').filter(|s| !s.is_empty()) {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let val = parts.next().unwrap_or("");

        match key {
            "tz" => {
                for tz_name in val.split(',').filter(|s| !s.is_empty()) {
                    let decoded = tz_name.trim().to_string();
                    if parse_tz(&decoded).is_some() && !timezones.contains(&decoded) {
                        timezones.push(decoded);
                    }
                }
            }
            "selected" => {
                if let Ok(h) = val.parse::<u32>() {
                    if h < 24 {
                        selected = Some(h);
                    }
                }
            }
            _ => {}
        }
    }

    (timezones, selected)
}

fn format_search_params(timezones: &[String], selected: Option<u32>) -> String {
    let mut parts = Vec::new();
    if !timezones.is_empty() {
        parts.push(format!("tz={}", timezones.join(",")));
    }
    if let Some(h) = selected {
        parts.push(format!("selected={}", h));
    }
    parts.join("&")
}

fn timezone_display_name(name: &str) -> String {
    if let Some(tz) = parse_tz(name) {
        let offset = format_offset(tz);
        let short = name.split('/').last().unwrap_or(name).replace('_', " ");
        format!("{} ({})", short, offset)
    } else {
        name.replace('_', " ")
    }
}

fn timezone_abbrev(name: &str) -> String {
    name.split('/').last().unwrap_or(name).replace('_', " ")
}

fn is_nighttime(tz: Tz, utc_hour: u32) -> bool {
    let (local_hour, _) = local_time_for_utc_hour(tz, utc_hour);
    local_hour < 7 || local_hour >= 22
}

fn format_12h(hour: u32) -> String {
    let h12 = match hour % 12 {
        0 => 12,
        h => h,
    };
    let suffix = if hour < 12 { "AM" } else { "PM" };
    format!("{}:00 {}", h12, suffix)
}

fn format_hour(hour: u32, use_12h: bool) -> String {
    if use_12h {
        format_12h(hour)
    } else {
        format!("{:02}:00", hour)
    }
}

fn format_current_time(tz: Tz, use_12h: bool) -> String {
    let now = Utc::now();
    let local = now.with_timezone(&tz);
    let h = local.hour();
    let m = local.minute();
    if use_12h {
        let h12 = match h % 12 { 0 => 12, h => h };
        let suffix = if h < 12 { "AM" } else { "PM" };
        format!("{}:{:02} {}", h12, m, suffix)
    } else {
        format!("{:02}:{:02}", h, m)
    }
}

#[component]
fn TimezoneCell(tz: Tz, utc_hour: u32, is_selected: bool, is_current: bool, use_12h: bool) -> Element {
    let (local_hour, day_offset) = local_time_for_utc_hour(tz, utc_hour);
    let night = is_nighttime(tz, utc_hour);

    let bg = if is_selected {
        "bg-accent/15"
    } else if is_current {
        "bg-primary/[0.06]"
    } else {
        ""
    };

    let fg = if is_selected {
        "text-accent-foreground"
    } else if is_current {
        "text-primary font-medium"
    } else if night {
        "text-muted-foreground/35"
    } else {
        "text-foreground/80"
    };

    let label = if day_offset == 0 {
        format_hour(local_hour, use_12h)
    } else {
        format!("{}{:+}", format_hour(local_hour, use_12h), day_offset)
    };

    rsx! {
        div { class: "flex items-center justify-center font-mono text-xs h-8 border-b border-border/25 transition-colors {bg} {fg}",
            "{label}"
        }
    }
}

#[component]
fn TimeLabel(utc_hour: u32, is_selected: bool, is_current: bool, use_12h: bool) -> Element {
    let txt = if is_selected {
        "text-accent font-semibold"
    } else if is_current {
        "text-primary font-semibold"
    } else {
        "text-muted-foreground"
    };

    let display = format_hour(utc_hour, use_12h);

    rsx! {
        div { class: "sticky left-0 z-10 flex items-center justify-end pr-3 font-mono text-xs h-8 border-b border-border/25 bg-card {txt}",
            "{display}"
        }
    }
}

#[component]
fn ColumnHeader(tz_name: String, on_remove: EventHandler<()>, use_12h: bool) -> Element {
    let display = timezone_abbrev(&tz_name);
    let offset = parse_tz(&tz_name).map(format_offset).unwrap_or_default();
    let current = parse_tz(&tz_name)
        .map(|tz| format_current_time(tz, use_12h))
        .unwrap_or_default();

    rsx! {
        div { class: "flex flex-col items-center gap-0.5 px-1.5 py-2 border-b border-border/50 bg-card shrink-0",
            div { class: "flex items-center gap-1",
                span { class: "text-sm font-semibold text-foreground truncate max-w-[90px]", "{display}" }
                button {
                    class: "size-4 flex items-center justify-center rounded-full text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors text-xs leading-none shrink-0",
                    onclick: move |_| on_remove.call(()),
                    "×"
                }
            }
            div { class: "flex items-center gap-1.5",
                span { class: "text-[10px] font-mono text-muted-foreground", "{offset}" }
                span { class: "text-xs font-mono text-primary", "{current}" }
            }
        }
    }
}


#[component]
pub fn App() -> Element {
    let mut timezones: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut selected_hour: Signal<Option<u32>> = use_signal(|| None);
    let mut show_picker: Signal<bool> = use_signal(|| false);
    let mut search: Signal<String> = use_signal(String::new);
    let use_12h: Signal<bool> = use_signal(|| false);
    let mut picker_origin: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let ready: Signal<bool> = use_signal(|| false);

    {
        let mut timezones = timezones.clone();
        let mut selected_hour = selected_hour.clone();
        let mut use_12h = use_12h.clone();
        let mut ready = ready.clone();
        use_effect(move || {
            let search = browser::get_search_params();
            let (tzs, sel) = parse_search_params(&search);

            if tzs.is_empty() {
                if let Some(local) = browser::detect_local_tz() {
                    timezones.set(vec![local]);
                } else {
                    timezones.set(vec!["UTC".to_string()]);
                }
            } else {
                timezones.set(tzs);
            }
            selected_hour.set(sel);
            use_12h.set(browser::prefers_12h());
            ready.set(true);
        });
    }

    {
        let timezones = timezones.clone();
        let selected_hour = selected_hour.clone();
        use_effect(move || {
            if !ready() {
                return;
            }
            let params = format_search_params(&timezones(), selected_hour());
            browser::set_search_params(&params);
        });
    }

    let now_hour = current_utc_hour();
    let sel_hour = selected_hour();
    let tz_list = timezones();
    let hour_12 = use_12h();

    let tz_header_data: Vec<(String, String)> = tz_list
        .clone()
        .into_iter()
        .map(|tz| {
            let tz_for_remove = tz.clone();
            (tz, tz_for_remove)
        })
        .collect();

    let row_data: Vec<(u32, bool, bool, String)> = (0..24)
        .map(|h| {
            let is_current = h == now_hour;
            let is_selected = sel_hour == Some(h);
            let row_bg = if is_selected {
                "bg-accent/8"
            } else if is_current {
                "bg-primary/[0.03]"
            } else {
                ""
            };
            (h, is_current, is_selected, row_bg.to_string())
        })
        .collect();

    let search_val = search();
    let tz_items: Vec<(String, String)> = if search_val.is_empty() {
        COMMON_TIMEZONES
            .iter()
            .map(|&tz| (timezone_display_name(tz), tz.to_string()))
            .collect()
    } else {
        filter_timezones(&search_val)
            .iter()
            .map(|&tz| (timezone_display_name(tz), tz.to_string()))
            .collect()
    };

    rsx! {
        div { class: "space-y-3",
            div { class: "relative overflow-x-auto rounded-lg border border-border bg-card",
                div { class: "min-w-[480px]",
                    div { class: "flex items-center",
                        div { class: "w-[72px] shrink-0" }
                        for (tz_name, tz_remove) in tz_header_data.clone() {
                            div { class: "w-[120px] shrink-0",
                                ColumnHeader {
                                    tz_name,
                                    on_remove: move |_| {
                                        let mut current = timezones();
                                        current.retain(|t| t != &tz_remove);
                                        if current.is_empty() {
                                            if let Some(local) = browser::detect_local_tz() {
                                                current.push(local);
                                            } else {
                                                current.push("UTC".to_string());
                                            }
                                        }
                                        timezones.set(current);
                                    },
                                    use_12h: hour_12,
                                }
                            }
                        }
                        div { class: "ml-auto flex items-center gap-2 px-3",
                            div { class: "relative",
                                button {
                                    id: "add-tz-btn",
                                    class: "inline-flex items-center gap-1 h-7 px-2.5 rounded-lg text-xs font-medium border border-border bg-background text-foreground/80 hover:bg-muted hover:text-foreground transition-colors whitespace-nowrap",
                                    onclick: move |_| {
                                        show_picker.toggle();
                                        if show_picker() {
                                            picker_origin.set(browser::get_picker_origin());
                                        }
                                    },
                                    "+ Add Timezone"
                                }
                            }
                        }
                    }
                    div { class: "relative",
                        for (utc_hour, is_current, is_selected, row_bg) in row_data {
                            div {
                                class: "flex {row_bg} cursor-pointer transition-colors hover:bg-muted/20",
                                onclick: move |_| {
                                    let current_sel = selected_hour();
                                    if current_sel == Some(utc_hour) {
                                        selected_hour.set(None);
                                    } else {
                                        selected_hour.set(Some(utc_hour));
                                    }
                                },
                                TimeLabel {
                                    utc_hour,
                                    is_selected,
                                    is_current,
                                    use_12h: hour_12,
                                }
                                for tz_name in tz_list.clone() {
                                    div { class: "w-[120px] shrink-0",
                                        if let Some(tz) = parse_tz(&tz_name) {
                                            TimezoneCell {
                                                tz,
                                                utc_hour,
                                                is_selected,
                                                is_current,
                                                use_12h: hour_12,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if show_picker() {
                SearchableDropdown {
                    search,
                    items: tz_items,
                    placeholder: "Search timezones...",
                    on_select: move |tz| {
                        let mut current = timezones();
                        if !current.contains(&tz) {
                            current.push(tz);
                            timezones.set(current);
                        }
                        show_picker.set(false);
                        search.set(String::new());
                    },
                    on_close: move |_| {
                        show_picker.set(false);
                        search.set(String::new());
                    },
                    origin: picker_origin(),
                }
            }
        }
    }
}
