use dioxus::prelude::*;

use crate::components::ui::icons::TriangleIcon;

const NAV: &[(&str, &[(&str, &str)])] = &[
    (
        "Foundations",
        &[
            ("Overview", "#overview"),
            ("Colors", "#colors"),
            ("Typography", "#typography"),
            ("Effects", "#effects"),
        ],
    ),
    (
        "Components",
        &[
            ("Buttons", "#buttons"),
            ("Badges", "#badges"),
            ("Inputs", "#inputs"),
            ("Cards", "#cards"),
        ],
    ),
];

#[component]
pub fn DsSidebar(class: Option<String>) -> Element {
    let mut classes = String::from(
        "flex h-full w-64 shrink-0 flex-col gap-8 border-r border-border bg-sidebar px-5 py-6",
    );
    if let Some(c) = class {
        classes.push(' ');
        classes.push_str(&c);
    }

    rsx! {
        aside {
            class: classes,
            a {
                href: "#overview",
                class: "flex items-center gap-2.5",
                span {
                    class: "flex size-8 items-center justify-center rounded-md bg-primary text-primary-foreground shadow-glow-pink",
                    TriangleIcon { class: "size-4", filled: true }
                }
                span {
                    class: "font-display text-sm font-bold tracking-widest text-foreground",
                    "OUTRUN"
                }
            }

            nav {
                class: "flex flex-col gap-7",
                for group in NAV.iter() {
                    div {
                        class: "flex flex-col gap-1.5",
                        p {
                            class: "px-2 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground",
                            "{group.0}"
                        }
                        ul {
                            class: "flex flex-col gap-0.5",
                            for item in group.1.iter() {
                                li {
                                    a {
                                        href: "{item.1}",
                                        class: "block rounded-md px-2 py-1.5 text-sm text-sidebar-foreground/80 transition-colors hover:bg-sidebar-accent hover:text-foreground",
                                        "{item.0}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "mt-auto rounded-lg border border-border bg-card/60 p-3",
                p {
                    class: "font-mono text-[0.7rem] uppercase tracking-widest text-accent",
                    "v1.0 — Neon"
                }
                p {
                    class: "mt-1 text-xs leading-relaxed text-muted-foreground",
                    "A retro-futuristic design system for the digital frontier."
                }
            }
        }
    }
}
