use dioxus::prelude::*;

use ui::icons::{BlueSkyIcon, GitBranchIcon, XIcon};

const LINKS: &[(&str, &str)] = &[
    ("/home", "/"),
    ("/about", "/about"),
    ("/projects", "/projects"),
    ("/blog", "/blog"),
];

const MORE_LINKS: &[(&str, &str)] = &[
    ("/how-i-work", "/how-i-work"),
    ("/labs", "/labs"),
    ("/resume", "/resume"),
    ("/music", "/music"),
];

const ALL_LINKS: &[(&str, &str)] = &[
    ("/home", "/"),
    ("/about", "/about"),
    ("/how-i-work", "/how-i-work"),
    ("/projects", "/projects"),
    ("/blog", "/blog"),
    ("/labs", "/labs"),
    ("/resume", "/resume"),
    ("/music", "/music"),
];

#[component]
pub fn PortfolioNav() -> Element {
    let mut more_open = use_signal(|| false);
    let mut mobile_open = use_signal(|| false);

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
                    div { class: "relative",
                        button {
                            class: "rounded-md px-2 py-1 font-mono text-xs text-muted-foreground transition-colors hover:text-accent hover:text-glow-cyan flex items-center gap-0.5",
                            onclick: move |_| more_open.toggle(),
                            "/more"
                            span { class: "text-[10px] opacity-60", "▾" }
                        }
                        if more_open() {
                            div {
                                class: "fixed inset-0 z-10",
                                onclick: move |_| more_open.set(false),
                            }
                            div { class: "absolute right-0 top-full z-20 mt-1 min-w-[120px] rounded-lg border border-border bg-card p-1 shadow-lg",
                                for (label, href) in MORE_LINKS {
                                    a {
                                        class: "block rounded-md px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground",
                                        href: "{href}",
                                        onclick: move |_| more_open.set(false),
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "hidden items-center gap-1 sm:flex",
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

                button {
                    class: "flex items-center rounded-md p-2 text-muted-foreground transition-colors hover:text-primary sm:hidden",
                    onclick: move |_| mobile_open.toggle(),
                    aria_label: "Menu",
                    svg {
                        class: "size-5",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        if mobile_open() {
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        } else {
                            line { x1: "4", y1: "6", x2: "20", y2: "6" }
                            line { x1: "4", y1: "12", x2: "20", y2: "12" }
                            line { x1: "4", y1: "18", x2: "20", y2: "18" }
                        }
                    }
                }
            }
        }

        if mobile_open() {
            div { class: "fixed inset-0 z-40 sm:hidden",
                div {
                    class: "absolute inset-0 bg-black/60 backdrop-blur-sm",
                    onclick: move |_| mobile_open.set(false),
                }
                div { class: "absolute right-0 top-0 z-50 h-full w-64 border-l border-border bg-background p-6",
                    div { class: "flex justify-end",
                        button {
                            class: "rounded-md p-2 text-muted-foreground transition-colors hover:text-primary",
                            onclick: move |_| mobile_open.set(false),
                            aria_label: "Close menu",
                            svg {
                                class: "size-5",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        }
                    }
                    nav { class: "mt-6 flex flex-col gap-1",
                        for (label, href) in ALL_LINKS {
                            a {
                                class: "rounded-md px-3 py-2 font-mono text-sm text-muted-foreground transition-colors hover:bg-muted hover:text-foreground",
                                href: "{href}",
                                onclick: move |_| mobile_open.set(false),
                                "{label}"
                            }
                        }
                    }
                    div { class: "mt-6 border-t border-border pt-4 flex items-center gap-2",
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
}
