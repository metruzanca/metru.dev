use dioxus::prelude::*;

#[component]
pub fn SearchableDropdown(
    search: Signal<String>,
    items: Vec<(String, String)>,
    placeholder: Option<String>,
    on_select: EventHandler<String>,
    on_close: EventHandler<()>,
    origin: Option<(f64, f64)>,
) -> Element {
    let ph = placeholder.unwrap_or_else(|| "Search...".to_string());

    let style = origin
        .map(|(x, y)| format!("position: fixed; left: {x}px; top: {y}px; z-index: 30;"))
        .unwrap_or_default();

    rsx! {
        div {
            class: "fixed inset-0 z-20",
            onclick: move |_| on_close.call(()),
        }
        div { class: "w-72 rounded-lg border border-border bg-popover shadow-xl overflow-hidden", style: "{style}",
            div { class: "p-2",
                input {
                    class: "h-8 w-full rounded-md border border-input bg-background px-2.5 text-sm text-foreground placeholder:text-muted-foreground outline-none focus:border-primary focus:shadow-glow-pink",
                    placeholder: "{ph}",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
            }
            div { class: "max-h-64 overflow-y-auto divide-y divide-border/20",
                for (display, value) in items.clone() {
                    div {
                        class: "flex items-center px-3 py-1.5 text-sm text-foreground/80 hover:bg-muted cursor-pointer transition-colors",
                        onclick: {
                            let value = value.clone();
                            move |_| on_select.call(value.clone())
                        },
                        "{display}"
                    }
                }
                if items.is_empty() && !search().is_empty() {
                    div { class: "px-3 py-3 text-sm text-muted-foreground text-center",
                        "No results found"
                    }
                }
            }
        }
    }
}
