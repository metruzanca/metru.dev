use dioxus::prelude::*;

use crate::blog;
use crate::Route;

#[derive(PartialEq, Clone)]
struct PageItem {
    title: String,
    description: String,
    url: String,
    section: &'static str,
}

fn static_pages() -> Vec<PageItem> {
    vec![
        PageItem {
            title: "Landing".into(),
            description: "Portfolio overview — projects, writing, music".into(),
            url: "/".into(),
            section: "Navigation",
        },
        PageItem {
            title: "Projects".into(),
            description: "Open source work and contributions on GitHub".into(),
            url: "/projects".into(),
            section: "Navigation",
        },
        PageItem {
            title: "Blog".into(),
            description: "Articles on Rust, web development, and technology".into(),
            url: "/blog".into(),
            section: "Navigation",
        },
        PageItem {
            title: "Music".into(),
            description: "What I'm listening to right now via Last.fm".into(),
            url: "/music".into(),
            section: "Navigation",
        },
        PageItem {
            title: "Design System".into(),
            description: "Component library and design token showcase".into(),
            url: "/design-system".into(),
            section: "Navigation",
        },
        PageItem {
            title: "Resume".into(),
            description: "Professional experience, skills, and work history".into(),
            url: "/resume".into(),
            section: "Navigation",
        },
    ]
}

fn all_pages() -> Vec<PageItem> {
    let mut pages = static_pages();

    for post in blog::published_posts() {
        pages.push(PageItem {
            title: post.frontmatter.title.clone(),
            description: post.frontmatter.description.clone(),
            url: format!("/blog/{}", post.slug),
            section: "Blog",
        });
    }

    pages
}

fn fuzzy_match(query: &str, text: &str) -> bool {
    let query = query.to_lowercase();
    let text = text.to_lowercase();
    let mut query_chars = query.chars().peekable();
    for c in text.chars() {
        if let Some(&qc) = query_chars.peek() {
            if c == qc {
                query_chars.next();
            }
        }
    }
    query_chars.peek().is_none()
}

fn match_score(query: &str, item: &PageItem) -> Option<usize> {
    if query.is_empty() {
        return Some(0);
    }

    if fuzzy_match(query, &item.title) {
        Some(0)
    } else if fuzzy_match(query, &item.description) {
        Some(1)
    } else {
        None
    }
}

const FOCUS_INPUT_JS: &str =
    "setTimeout(function(){var e=document.querySelector('[data-cmd-input]');if(e)e.focus()},0)";

#[component]
pub fn CommandPalette(mut open: Signal<bool>) -> Element {
    let mut query = use_signal(String::new);
    let mut selected = use_signal(|| 0usize);

    let filtered = use_memo(move || {
        let q = query().trim().to_string();
        let mut scored: Vec<(usize, PageItem)> = all_pages()
            .into_iter()
            .filter_map(|item| match_score(&q, &item).map(|score| (score, item)))
            .collect();
        scored.sort_by_key(|(score, _)| *score);
        scored
    });

    let nav = use_navigator();

    let mut close = move || {
        open.set(false);
        query.set(String::new());
        selected.set(0);
    };

    let mut navigate = move |url: &str| {
        match url {
            "/" => { nav.push(Route::Landing {}); }
            "/projects" => { nav.push(Route::ProjectsList {}); }
            "/blog" => { nav.push(Route::BlogList {}); }
            "/music" => { nav.push(Route::Music {}); }
            "/design-system" => { nav.push(Route::DesignSystem {}); }
            "/resume" => { nav.push(Route::ResumePage {}); }
            u if u.starts_with("/blog/") => {
                let slug = u.strip_prefix("/blog/").unwrap_or("");
                nav.push(Route::BlogPost {
                    slug: slug.to_string(),
                });
            }
            _ => {}
        }
        close();
    };

    use_effect(move || {
        if open() {
            let _ = dioxus::document::eval(FOCUS_INPUT_JS);
        }
    });

    use_effect(move || {
        let idx = selected();
        if open() {
            let js = format!(
                "var e=document.querySelector('[data-cmd-item=\"{}\"]');if(e)e.scrollIntoView({{block:'nearest'}})",
                idx
            );
            let _ = dioxus::document::eval(&js);
        }
    });

    if !open() {
        return rsx! {};
    }

    let items = filtered();
    let sel = selected();

    rsx! {
        div {
            class: "fixed inset-0 z-50",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    e.prevent_default();
                    close();
                }
            },
            div {
                class: "fixed inset-0 bg-black/60 backdrop-blur-sm",
                onclick: move |_| close(),
                div { class: "fixed inset-0 overflow-y-auto pointer-events-none",
                    div { class: "flex min-h-full items-start justify-center p-4 pt-[15vh]",
                        div {
                            class: "w-full max-w-xl rounded-xl border border-border bg-card shadow-glow-pink overflow-hidden pointer-events-auto",
                            onclick: move |e| e.stop_propagation(),
                            div { class: "flex items-center border-b border-border px-4",
                                svg {
                                    class: "size-4 shrink-0 text-muted-foreground",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                                    }
                                }
                                input {
                                    class: "flex-1 h-12 bg-transparent px-3 text-sm text-foreground placeholder:text-muted-foreground outline-none",
                                    placeholder: "Search pages and posts...",
                                    value: "{query}",
                                    "data-cmd-input": "true",
                                    oninput: move |e| {
                                        query.set(e.value());
                                        selected.set(0);
                                    },
                                    onblur: move |_| {
                                        if open() {
                                            let _ = dioxus::document::eval(FOCUS_INPUT_JS);
                                        }
                                    },
                                    onkeydown: move |e| {
                                        match e.key() {
                                            Key::ArrowDown => {
                                                e.prevent_default();
                                                let total = items.len();
                                                if total > 0 && selected() + 1 < total {
                                                    *selected.write() += 1;
                                                }
                                            }
                                            Key::ArrowUp => {
                                                e.prevent_default();
                                                if selected() > 0 {
                                                    *selected.write() -= 1;
                                                }
                                            }
                                            Key::Enter => {
                                                e.prevent_default();
                                                if let Some((_, item)) = items.get(selected()) {
                                                    let url = item.url.clone();
                                                    navigate(&url);
                                                }
                                            }
                                            Key::Escape => {
                                                e.prevent_default();
                                                close();
                                            }
                                            _ => {}
                                        }
                                    },
                                }
                                kbd {
                                    class: "ml-2 rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground",
                                    "esc"
                                }
                            }
                            div { class: "max-h-80 overflow-y-auto p-2",
                                if items.is_empty() && !query.read().trim().is_empty() {
                                    div { class: "px-4 py-8 text-center text-sm text-muted-foreground",
                                        "No results found."
                                    }
                                } else {
                                    for (i, (_, item)) in items.iter().enumerate() {
                                        {
                                            let show_header = i == 0 || items[i - 1].1.section != item.section;
                                            let url = item.url.clone();
                                            let title = item.title.clone();
                                            let desc = item.description.clone();
                                            let url_display = item.url.clone();
                                            let heading = if show_header { Some(item.section) } else { None };
                                            rsx! {
                                                if let Some(section_name) = heading {
                                                    div { class: "pt-3 pb-1 first:pt-0",
                                                        div { class: "font-mono text-[10px] uppercase tracking-widest text-muted-foreground/60",
                                                            "{section_name}"
                                                        }
                                                    }
                                                }
                                                button {
                                                    "data-cmd-item": "{i}",
                                                    class: if i == sel {
                                                        "w-full rounded-lg px-4 py-3 text-left transition-colors bg-accent/10"
                                                    } else {
                                                        "w-full rounded-lg px-4 py-3 text-left transition-colors hover:bg-muted"
                                                    },
                                                    onmouseenter: move |_| selected.set(i),
                                                    onmousedown: move |e| e.prevent_default(),
                                                    onclick: move |_| {
                                                        navigate(&url);
                                                    },
                                                    div { class: "flex items-start justify-between gap-4",
                                                        div { class: "min-w-0 flex-1",
                                                            div { class: "truncate text-sm font-medium",
                                                                "{title}"
                                                            }
                                                            if !desc.is_empty() {
                                                                div { class: "mt-0.5 truncate text-xs text-muted-foreground",
                                                                    "{desc}"
                                                                }
                                                            }
                                                        }
                                                        div { class: "shrink-0 pt-0.5 font-mono text-[11px] text-accent/70",
                                                            "{url_display}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex items-center justify-between border-t border-border px-4 py-2",
                                span { class: "font-mono text-[10px] text-muted-foreground",
                                    "Navigate with arrow keys"
                                }
                                span { class: "font-mono text-[10px] text-muted-foreground",
                                    "Enter to select"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
