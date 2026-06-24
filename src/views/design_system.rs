use dioxus::prelude::*;

use crate::components::ds::{DsSidebar, DsTopbar, DsHero, DsFoundations, DsComponentsShowcase};

#[component]
pub fn DesignSystem() -> Element {
    rsx! {
        div {
            class: "flex min-h-screen",
            DsSidebar {
                class: "sticky top-0 hidden h-screen lg:flex",
            }
            main {
                class: "flex min-w-0 flex-1 flex-col",
                DsTopbar {}
                DsHero {}
                div { class: "h-px bg-border" }
                DsFoundations {}
                div { class: "h-px bg-border" }
                DsComponentsShowcase {}
                footer {
                    class: "border-t border-border px-4 py-8 md:px-8",
                    div {
                        class: "flex flex-col items-start justify-between gap-3 sm:flex-row sm:items-center",
                        p {
                            class: "font-display text-sm font-bold uppercase tracking-widest text-foreground",
                            "Outrun"
                        }
                        p {
                            class: "font-mono text-xs text-muted-foreground",
                            "Synthwave Design System — built for the digital frontier"
                        }
                    }
                }
            }
        }
    }
}
