use dioxus::prelude::*;

use crate::blog;
use crate::Route;
use super::section_heading::{PortfolioSectionHeading, SectionAction};

#[component]
pub fn PortfolioWriting() -> Element {
    let posts = blog::published_posts();
    let nav = use_navigator();

    rsx! {
        section { id: "writing", class: "px-4 py-12",
            PortfolioSectionHeading {
                index: "02".to_string(),
                label: "writing".to_string(),
                action: Some(SectionAction {
                    label: "view all".to_string(),
                    href: "/blog".to_string(),
                }),
            }

            ul { class: "flex flex-col",
                for post in posts.iter().take(4) {
                    li {
                        a {
                            class: "group flex items-baseline gap-4 border-b border-border py-3.5 transition-colors hover:border-accent/50",
                            href: "#",
                            onclick: {
                                let slug = post.slug.clone();
                                move |e| {
                                    e.prevent_default();
                                    nav.push(Route::BlogPost { slug: slug.clone() });
                                }
                            },
                            time { class: "hidden shrink-0 font-mono text-xs text-muted-foreground sm:block",
                                "{post.frontmatter.timestamp.split('T').next().unwrap_or(&post.frontmatter.timestamp)}"
                            }
                            span { class: "flex-1 text-pretty text-sm text-foreground transition-colors group-hover:text-accent group-hover:text-glow-cyan md:text-base",
                                "{post.frontmatter.title}"
                            }
                            span { class: "shrink-0 font-mono text-xs text-muted-foreground",
                                "{estimate_read_time(&post.body_markdown)} min"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn estimate_read_time(body: &str) -> usize {
    let words = body.split_whitespace().count();
    (words / 200).max(1)
}
