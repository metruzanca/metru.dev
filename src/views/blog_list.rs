use dioxus::prelude::*;

use crate::blog;
use crate::components::portfolio::WritingItem;
use crate::server_functions::get_blog_posts;

#[component]
pub fn BlogList() -> Element {
    let merged = use_server_future(|| async {
        get_blog_posts().await.unwrap_or_default()
    })?;

    let posts = use_memo(move || {
        merged().unwrap_or_else(|| blog::published_posts_owned())
    });

    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pb-12 pt-14 md:pt-20",
            h1 { class: "mb-8 font-display text-3xl font-extrabold uppercase text-foreground md:text-4xl",
                "Blog"
            }
            if posts.read().is_empty() {
                div { class: "rounded-lg border border-border p-12 text-center",
                    p { class: "text-muted-foreground",
                        "No posts yet. Check back soon."
                    }
                }
            } else {
                ul { class: "flex flex-col",
                    for post in posts.read().iter() {
                        WritingItem {
                            post: post.clone(),
                            expanded: true,
                        }
                    }
                }
            }

            footer { class: "border-t border-border px-4 py-8 mt-12",
                div { class: "flex flex-col items-start justify-between gap-2 sm:flex-row sm:items-center",
                    p { class: "font-mono text-xs text-muted-foreground",
                        span { class: "text-primary", "$" }
                        " built with synthwave \u{2014} \u{a9} 2026 Sam"
                    }
                }
            }
        }
    }
}
