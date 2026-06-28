use dioxus::prelude::*;

use crate::components::ui::icons::{GitBranchIcon, AtSignIcon, SendIcon};

const LINKS: &[(&str, &str)] = &[
    ("~/", "#top"),
    ("/projects", "#projects"),
    ("/writing", "#writing"),
    ("/activity", "#activity"),
    ("/contact", "#contact"),
];

#[component]
pub fn PortfolioNav() -> Element {
    rsx! {
        header { class: "sticky top-0 z-30 border-b border-border bg-background/80 backdrop-blur-md",
            div { class: "mx-auto flex max-w-3xl items-center justify-between gap-4 px-4 py-3",
                a {
                    class: "flex items-center gap-2 font-mono text-sm text-foreground",
                    href: "#top",
                    span { class: "inline-block size-2 rounded-full bg-primary shadow-glow-pink" }
                    span { class: "font-semibold", "sam" }
                    span { class: "text-muted-foreground", "\u{b7} dev" }
                }

                nav { class: "hidden items-center gap-1 sm:flex", aria_label: "Primary",
                    for (label, href) in LINKS {
                        a {
                            class: "rounded-md px-2 py-1 font-mono text-xs text-muted-foreground transition-colors hover:text-accent hover:text-glow-cyan",
                            href: "{href}",
                            "{label}"
                        }
                    }
                }

                div { class: "flex items-center gap-1",
                    a {
                        class: "rounded-md p-1.5 text-muted-foreground transition-colors hover:text-primary",
                        href: "https://github.com",
                        target: "_blank",
                        rel: "noreferrer",
                        aria_label: "GitHub",
                        GitBranchIcon { class: "size-4" }
                    }
                    a {
                        class: "rounded-md p-1.5 text-muted-foreground transition-colors hover:text-primary",
                        href: "https://x.com",
                        target: "_blank",
                        rel: "noreferrer",
                        aria_label: "X / Twitter",
                        AtSignIcon { class: "size-4" }
                    }
                    a {
                        class: "rounded-md p-1.5 text-muted-foreground transition-colors hover:text-primary",
                        href: "#contact",
                        aria_label: "Contact",
                        SendIcon { class: "size-4" }
                    }
                }
            }
        }
    }
}
