use dioxus::prelude::*;

use crate::labs;
use crate::Route;

#[component]
pub fn LabsList() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pb-12 pt-14 md:pt-20",
            h1 { class: "mb-2 font-display text-3xl font-extrabold uppercase text-foreground md:text-4xl",
                "Labs"
            }
            p { class: "mb-8 text-sm leading-relaxed text-muted-foreground",
                "Small tools, experiments, and utilities built with Dioxus."
            }

            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                for lab in labs::LABS {
                    LabCard { lab }
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

fn tag_accent(index: usize) -> &'static str {
    if index % 2 == 0 {
        "primary"
    } else {
        "cyan"
    }
}

fn tag_classes(accent: &str) -> &'static str {
    match accent {
        "cyan" => "rounded-full border px-2.5 py-0.5 font-mono text-[0.65rem] border-accent/40 text-accent",
        _ => "rounded-full border px-2.5 py-0.5 font-mono text-[0.65rem] border-primary/40 text-primary",
    }
}

#[component]
fn LabCard(lab: &'static crate::labs::LabInfo) -> Element {
    let tags: Vec<&str> = lab.meta.tags.iter().take(4).copied().collect();

    rsx! {
        Link {
            class: "group relative flex flex-col rounded-xl border border-border bg-card p-5 transition-colors hover:border-accent/50",
            to: Route::LabPage { slug: lab.meta.slug.to_string() },
            div { class: "flex items-start justify-between gap-3",
                h3 { class: "font-display text-lg font-bold uppercase tracking-tight text-foreground transition-colors group-hover:text-accent",
                    "{lab.meta.name}"
                }
            }

            p { class: "mt-2 flex-1 text-sm leading-relaxed text-muted-foreground",
                "{lab.meta.description}"
            }

            if !tags.is_empty() {
                div { class: "mt-4 flex flex-wrap gap-1.5",
                    for (i, tag) in tags.iter().enumerate() {
                        span {
                            class: "{tag_classes(tag_accent(i))}",
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}
