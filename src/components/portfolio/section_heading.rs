use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct SectionAction {
    pub label: String,
    pub href: String,
}

#[component]
pub fn PortfolioSectionHeading(
    index: String,
    label: String,
    action: Option<SectionAction>,
) -> Element {
    rsx! {
        div { class: "mb-6 flex items-baseline justify-between gap-4 border-b border-border pb-2",
            h2 { class: "font-mono text-sm text-muted-foreground",
                span { class: "text-primary", "{index}" }
                " // "
                span { class: "text-foreground", "{label}" }
            }
            if let Some(action) = action {
                a {
                    class: "font-mono text-xs text-muted-foreground transition-colors hover:text-accent hover:text-glow-cyan",
                    href: "{action.href}",
                    "{action.label} \u{2192}"
                }
            }
        }
    }
}
