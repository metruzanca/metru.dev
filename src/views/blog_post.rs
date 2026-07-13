use dioxus::prelude::*;

use crate::blog;
use crate::server_functions::get_blog_posts;
use crate::Route;

#[component]
pub fn BlogPost(slug: String) -> Element {
    let nav = use_navigator();
    let live = use_resource(|| async { get_blog_posts().await.unwrap_or_default() });

    let post = use_memo(move || {
        let slug = slug.clone();
        // Try live data first
        if let Some(ref posts) = live() {
            posts.iter().find(|p| p.slug == slug).cloned()
        } else {
            // Fall back to snapshot while live is loading
            blog::post_by_slug(&slug).cloned()
        }
    });

    let live_loaded = use_memo(move || live().is_some());

    match (post(), live_loaded()) {
        (Some(post), _) => {
            let frontmatter = &post.frontmatter;
            let html = blog::render_markdown(&post.body_markdown);

            rsx! {
                article { class: "mx-auto max-w-3xl px-6 pt-12 pb-24",
                        header { class: "mb-12",
                            time {
                                class: "mb-3 block font-mono text-sm text-muted-foreground",
                                datetime: "{frontmatter.timestamp}",
                                "{frontmatter.timestamp.split('T').next().unwrap_or(&frontmatter.timestamp)}"
                            }

                            h1 { class: "font-heading text-4xl font-extrabold tracking-tight text-glow-pink leading-tight",
                                "{frontmatter.title}"
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

                        div {
                            class: "prose-content",
                            dangerous_inner_html: html,
                        }
                    }
            }
        }
        (None, true) => {
            rsx! {
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
            }
        }
        (None, false) => {
            rsx! {
                div { class: "flex items-center justify-center py-24",
                    div { class: "text-center",
                        p { class: "text-muted-foreground", "Loading..." }
                    }
                }
            }
        }
    }
}
