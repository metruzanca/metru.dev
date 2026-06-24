use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant, ButtonSize};
use crate::components::ui::icons::{SearchIcon, BookOpenIcon, CommandIcon};

#[component]
pub fn DsTopbar() -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-20 flex items-center gap-4 border-b border-border bg-background/80 px-4 py-3 backdrop-blur-md md:px-8",
            div {
                class: "relative flex w-full max-w-md items-center",
                SearchIcon { class: "pointer-events-none absolute left-3 size-4 text-muted-foreground" }
                input {
                    r#type: "search",
                    placeholder: "Search the system",
                    "aria-label": "Search the design system",
                    class: "h-9 w-full rounded-md border border-input bg-card/60 pl-9 pr-16 text-sm text-foreground placeholder:text-muted-foreground outline-none transition-colors focus:border-primary focus:shadow-glow-pink",
                }
                kbd {
                    class: "absolute right-2.5 hidden items-center gap-1 rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[0.65rem] text-muted-foreground sm:flex",
                    CommandIcon { class: "size-3" }
                    "K"
                }
            }

            div {
                class: "ml-auto flex items-center gap-2",
                span {
                    class: "hidden items-center gap-1.5 rounded-full border border-accent/40 bg-accent/10 px-2.5 py-1 font-mono text-[0.7rem] uppercase tracking-widest text-accent sm:flex",
                    span { class: "size-1.5 rounded-full bg-accent shadow-glow-cyan" }
                    "Live"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::IconSm,
                    aria_label: "Documentation",
                    BookOpenIcon { class: "size-4" }
                }
                Button {
                    variant: ButtonVariant::Default,
                    size: ButtonSize::Sm,
                    "Get Started"
                }
            }
        }
    }
}
