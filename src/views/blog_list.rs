use dioxus::prelude::*;

use crate::blog;
use crate::Route;

#[component]
pub fn BlogList() -> Element {
    let posts = blog::published_posts();
    let nav = use_navigator();

    rsx! {
        div { class: "mx-auto max-w-3xl px-6 py-24",
            header { class: "mb-16",
                    h1 { class: "font-heading text-4xl font-extrabold tracking-tight text-glow-pink",
                        "metru.dev"
                    }
                    p { class: "mt-4 text-lg text-muted-foreground",
                        "Thoughts on programming, architecture, and the craft of software."
                    }
                }

                if posts.is_empty() {
                    div { class: "rounded-lg border border-border p-12 text-center",
                        p { class: "text-muted-foreground",
                            "No posts yet. Check back soon."
                        }
                    }
                } else {
                    div { class: "space-y-10",
                        for post in &posts {
                            article {
                                class: "group relative rounded-lg border border-border bg-card p-6 transition-shadow hover:shadow-glow-pink/20",
                            onclick: {
                                let slug = post.slug.clone();
                                move |_| {
                                    nav.push(Route::BlogPost { slug: slug.clone() });
                                }
                            },

                                time {
                                    class: "mb-2 block font-mono text-sm text-muted-foreground",
                                    datetime: "{post.frontmatter.timestamp}",
                                    "{post.frontmatter.timestamp.split('T').next().unwrap_or(&post.frontmatter.timestamp)}"
                                }

                                h2 { class: "mb-2 font-heading text-xl font-semibold tracking-tight text-foreground group-hover:text-neon-pink transition-colors",
                                    "{post.frontmatter.title}"
                                }

                                p { class: "mb-4 leading-relaxed text-muted-foreground",
                                    "{post.frontmatter.description}"
                                }

                                if !post.frontmatter.tags.is_empty() {
                                    div { class: "flex flex-wrap gap-2",
                                        for tag in &post.frontmatter.tags {
                                            span { class: "inline-flex items-center rounded-md border border-border bg-secondary px-2 py-0.5 font-mono text-xs text-secondary-foreground",
                                                "#{tag}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
        }
    }
}
