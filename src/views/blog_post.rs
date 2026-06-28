use dioxus::prelude::*;

use crate::blog;
use crate::Route;

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = blog::post_by_slug(&slug);
    let nav = use_navigator();

    match post {
        None => rsx! {
            div { class: "flex items-center justify-center py-24",
                div { class: "text-center",
                    h1 { class: "font-heading text-2xl text-foreground", "Post not found" }
                    p { class: "mt-2 text-muted-foreground", "The post you're looking for doesn't exist or hasn't been published yet." }
                    a {
                        class: "mt-6 inline-block text-neon-cyan hover:underline",
                        href: "/blog",
                        onclick: move |e| {
                            e.prevent_default();
                            nav.push(Route::BlogList {});
                        },
                        "Back to blog"
                    }
                }
            }
        },
        Some(post) => {
            let frontmatter = &post.frontmatter;
            let html = blog::render_markdown(&post.body_markdown);

            rsx! {
                article { class: "mx-auto max-w-3xl px-6 py-24",
                        header { class: "mb-12",
                            a {
                                class: "mb-6 inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-neon-cyan transition-colors",
                                href: "/blog",
                                onclick: move |e| {
                                    e.prevent_default();
                                    nav.push(Route::BlogList {});
                                },
                                span { class: "text-lg leading-none", "\u{2190}" }
                                " Blog"
                            }

                            time {
                                class: "mb-3 block font-mono text-sm text-muted-foreground",
                                datetime: "{frontmatter.timestamp}",
                                "{frontmatter.timestamp.split('T').next().unwrap_or(&frontmatter.timestamp)}"
                            }

                            h1 { class: "font-heading text-4xl font-extrabold tracking-tight text-glow-pink leading-tight",
                                "{frontmatter.title}"
                            }

                            p { class: "mt-4 text-lg leading-relaxed text-muted-foreground",
                                "{frontmatter.description}"
                            }

                            if !frontmatter.tags.is_empty() {
                                div { class: "mt-6 flex flex-wrap gap-2",
                                    for tag in &frontmatter.tags {
                                        span { class: "inline-flex items-center rounded-md border border-border bg-secondary px-2 py-0.5 font-mono text-xs text-secondary-foreground",
                                            "#{tag}"
                                        }
                                    }
                                }
                            }
                        }

                        // Blog content rendered from markdown
                        div {
                            class: "prose-content",
                            dangerous_inner_html: html,
                        }
                    }
            }
        }
    }
}
