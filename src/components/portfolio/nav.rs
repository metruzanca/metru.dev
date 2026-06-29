use dioxus::prelude::*;

use crate::components::ui::icons::{BlueSkyIcon, GitBranchIcon, XIcon};

const LINKS: &[(&str, &str)] = &[("/projects", "/projects"), ("/blog", "/blog")];

#[component]
pub fn PortfolioNav() -> Element {
    rsx! {
        header { class: "sticky top-0 z-30 border-b border-border bg-background/80 backdrop-blur-md",
            div { class: "mx-auto flex max-w-3xl items-center justify-between gap-4 px-4 py-3",
                a {
                    class: "flex items-center gap-2 font-mono text-sm text-foreground",
                    href: "/",
                    span { class: "inline-block size-2 rounded-full bg-primary shadow-glow-pink" }
                    span { class: "font-semibold font-display", "SAM" }
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
                        href: "https://github.com/metruzanca",
                        target: "_blank",
                        rel: "noreferrer",
                        aria_label: "GitHub",
                        GitBranchIcon { class: "size-4" }
                    }
                    a {
                        class: "rounded-md p-1.5 text-muted-foreground transition-colors hover:text-primary",
                        href: "https://x.com/metruzanca",
                        target: "_blank",
                        rel: "noreferrer",
                        aria_label: "X / Twitter",
                        XIcon { class: "size-4" }
                    }
                    a {
                        class: "rounded-md p-1.5 text-muted-foreground transition-colors hover:text-primary",
                        href: "https://bsky.app/profile/metru.dev",
                        target: "_blank",
                        rel: "noreferrer",
                        aria_label: "BlueSky",
                        BlueSkyIcon { class: "size-4" }
                    }
                }
            }
        }
    }
}
