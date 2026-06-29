use dioxus::prelude::*;

use crate::blog;
use crate::components::portfolio::WritingItem;

#[component]
pub fn BlogList() -> Element {
    let posts = blog::published_posts();

    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pb-12 pt-14 md:pt-20",
            if posts.is_empty() {
                div { class: "rounded-lg border border-border p-12 text-center",
                    p { class: "text-muted-foreground",
                        "No posts yet. Check back soon."
                    }
                }
            } else {
                ul { class: "flex flex-col",
                    for post in &posts {
                        WritingItem {
                            post: (*post).clone(),
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
                    p { class: "font-mono text-xs text-muted-foreground",
                        "designed in the neon"
                    }
                }
            }
        }
    }
}
