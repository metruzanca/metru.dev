use dioxus::prelude::*;

use crate::components::ui::icons::{ArrowUpRightIcon, StarIcon};
use super::section_heading::{PortfolioSectionHeading, SectionAction};

struct Project {
    name: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    stars: &'static str,
    accent: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        name: "Neon ORM",
        description: "A type-safe query builder for Postgres with zero-runtime overhead and a delightful API.",
        tags: &["TypeScript", "Postgres"],
        stars: "742",
        accent: "primary",
    },
    Project {
        name: "Gridrunner",
        description: "Real-time collaborative canvas with conflict-free sync built on CRDTs and WebRTC.",
        tags: &["Rust", "WASM"],
        stars: "318",
        accent: "cyan",
    },
    Project {
        name: "Pulse CLI",
        description: "An observability toolkit that turns raw logs into live, queryable dashboards in your terminal.",
        tags: &["Go", "DX"],
        stars: "203",
        accent: "primary",
    },
    Project {
        name: "Synthwave UI",
        description: "The retro-futuristic component library powering this very site. Accessible and glowing.",
        tags: &["React", "Design"],
        stars: "156",
        accent: "cyan",
    },
];

fn tag_classes(accent: &str) -> &'static str {
    match accent {
        "cyan" => "rounded-md border px-2 py-0.5 font-mono text-[0.7rem] border-accent/40 text-accent",
        _ => "rounded-md border px-2 py-0.5 font-mono text-[0.7rem] border-primary/40 text-primary",
    }
}

#[component]
pub fn PortfolioProjects() -> Element {
    rsx! {
        section { id: "projects", class: "px-4 py-12",
            PortfolioSectionHeading {
                index: "01".to_string(),
                label: "building".to_string(),
                action: Some(SectionAction {
                    label: "all projects".to_string(),
                    href: "#".to_string(),
                }),
            }

            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                for project in PROJECTS {
                    a {
                        class: "group relative flex flex-col rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
                        href: "#",
                        div { class: "flex items-start justify-between gap-3",
                            h3 { class: "font-display text-lg font-bold uppercase tracking-tight text-foreground",
                                "{project.name}"
                            }
                            ArrowUpRightIcon { class: "size-4 shrink-0 text-muted-foreground transition-colors group-hover:text-primary" }
                        }

                        p { class: "mt-2 flex-1 text-sm leading-relaxed text-muted-foreground",
                            "{project.description}"
                        }

                        div { class: "mt-4 flex items-center justify-between gap-3",
                            div { class: "flex flex-wrap gap-1.5",
                                for tag in project.tags {
                                    span {
                                        class: "{tag_classes(project.accent)}",
                                        "{tag}"
                                    }
                                }
                            }
                            span { class: "inline-flex items-center gap-1 font-mono text-xs text-muted-foreground",
                                StarIcon { class: "size-3.5" }
                                "{project.stars}"
                            }
                        }
                    }
                }
            }
        }
    }
}
