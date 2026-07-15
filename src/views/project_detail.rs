use dioxus::prelude::*;

use crate::blog::content;
use crate::projects;
use crate::Route;

#[component]
pub fn ProjectCaseStudy(slug: String) -> Element {
    let nav = use_navigator();

    let project = projects::project_by_slug(&slug);

    match project {
        Some(project) => {
            let fm = &project.frontmatter;
            let html = use_memo(move || content::render_blocks(&project.body, "projects"));

            rsx! {
                article { class: "mx-auto max-w-3xl px-6 pt-12 pb-24",
                    Link {
                        to: Route::ProjectsList {},
                        class: "inline-flex items-center gap-1 font-mono text-xs text-accent hover:underline mb-6",
                        "\u{2190} All Projects"
                    }

                    header { class: "mb-12",
                        time {
                            class: "mb-3 block font-mono text-sm text-muted-foreground",
                            datetime: "{fm.timestamp}",
                            "{fm.timestamp.split('T').next().unwrap_or(&fm.timestamp)}"
                        }

                        h1 { class: "font-heading text-4xl font-extrabold tracking-tight text-glow-pink leading-tight",
                            "{fm.title}"
                        }

                        if let Some(company) = &fm.company {
                            p { class: "mt-2 font-mono text-sm text-accent",
                                "{company}"
                            }
                        }

                        if !fm.github.is_empty() {
                            a {
                                class: "mt-3 inline-flex items-center gap-1.5 rounded-lg border border-border bg-card px-4 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-accent hover:text-accent",
                                href: "{fm.github}",
                                target: "_blank",
                                rel: "noreferrer",
                                "View source on GitHub \u{2192}"
                            }
                        }

                        if !fm.tags.is_empty() {
                            div { class: "mt-6 flex flex-wrap gap-2",
                                for tag in &fm.tags {
                                    span { class: "inline-flex items-center rounded-md border border-border bg-secondary px-2 py-0.5 font-mono text-xs text-secondary-foreground",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "prose-content",
                        dangerous_inner_html: html(),
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "flex items-center justify-center py-24",
                    div { class: "text-center",
                        h1 { class: "font-heading text-2xl text-foreground", "Case study not found" }
                        p { class: "mt-2 text-muted-foreground", "The case study you're looking for doesn't exist." }
                        a {
                            class: "mt-6 inline-block text-neon-cyan hover:underline",
                            href: "/projects",
                            onclick: move |e| {
                                e.prevent_default();
                                nav.push(Route::ProjectsList {});
                            },
                            "Back to projects"
                        }
                    }
                }
            }
        }
    }
}
