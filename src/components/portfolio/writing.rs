use dioxus::prelude::*;

use crate::blog;


#[component]
pub fn PortfolioWriting() -> Element {
    let posts = blog::published_posts();

    rsx! {
        section { id: "writing", class: "px-4 py-12",
            div { class: "mb-6 flex items-center justify-end border-b border-border pb-2",
                a {
                    class: "font-mono text-xs text-muted-foreground transition-colors hover:text-accent hover:text-glow-cyan",
                    href: "/blog",
                    "view all \u{2192}"
                }
            }

            ul { class: "flex flex-col",
                for post in posts.iter().take(4) {
                    WritingItem { post: (*post).clone() }
                }
            }
        }
    }
}

#[component]
pub fn WritingItem(post: blog::BlogPost, expanded: Option<bool>) -> Element {
    let slug = post.slug.clone();
    let date = format_post_date(&post.frontmatter.timestamp);
    let read_time = estimate_read_time(&post.body_markdown);
    let is_expanded = expanded.unwrap_or(false);

    rsx! {
        li { class: "group border-b border-border transition-colors hover:border-accent/50",
            a {
                class: "flex items-baseline gap-4 py-3.5 cursor-pointer",
                href: "/blog/{slug}",
                time { class: "hidden w-28 shrink-0 font-mono text-xs text-muted-foreground sm:block",
                    "{date}"
                }
                span { class: "flex-1 text-pretty text-sm text-foreground transition-colors group-hover:text-accent group-hover:text-glow-cyan md:text-base",
                    "{post.frontmatter.title}"
                }
                span { class: "w-14 shrink-0 text-right font-mono text-xs text-muted-foreground",
                    "{read_time} min"
                }
            }
            div { class: if is_expanded { "" } else { "overflow-hidden transition-all duration-200 max-h-0 group-hover:max-h-24" },
                div { class: "flex gap-4 pb-3.5",
                    span { class: "hidden w-28 shrink-0 sm:block" }
                    div { class: "flex-1",
                        if !post.frontmatter.description.is_empty() {
                            p { class: "text-sm leading-relaxed text-muted-foreground",
                                "{post.frontmatter.description}"
                            }
                        }
                        if !post.frontmatter.tags.is_empty() {
                            div { class: "mt-2 flex flex-wrap gap-1.5",
                                for tag in &post.frontmatter.tags {
                                    span { class: "rounded-md border border-border/60 px-2 py-0.5 font-mono text-[0.65rem] text-muted-foreground",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }
                    span { class: "hidden w-14 shrink-0 sm:block" }
                }
            }
        }
    }
}

fn format_post_date(iso: &str) -> String {
    let date = iso.split('T').next().unwrap_or(iso);
    if date.len() < 10 { return date.to_string(); }
    let month = match &date[5..7] {
        "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
        "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
        "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
        _ => "",
    };
    let year = &date[0..4];
    let day: &str = &date[8..10];
    let day_num: u32 = day.parse().unwrap_or(0);
    format!("{month} {day_num:02}, {year}")
}

fn estimate_read_time(body: &str) -> usize {
    let words = body.split_whitespace().count();
    (words / 200).max(1)
}
