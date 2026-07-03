use dioxus::prelude::*;

use crate::labs;
use crate::Route;

#[component]
pub fn LabPage(slug: String) -> Element {
    let lab = labs::LABS.iter().find(|l| l.meta.slug == slug);

    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pb-12 pt-14 md:pt-20",
            match lab {
                Some(info) => {
                    rsx! {
                        Link {
                            to: Route::LabsList {},
                            class: "inline-flex items-center gap-1 font-mono text-xs text-accent hover:underline mb-6",
                            "\u{2190} All Labs"
                        }
                        h1 { class: "mb-2 font-display text-3xl font-extrabold uppercase text-foreground md:text-4xl",
                            "{info.meta.name}"
                        }
                        p { class: "mb-8 text-sm leading-relaxed text-muted-foreground",
                            "{info.meta.description}"
                        }
                        div { class: "rounded-xl border border-border bg-card p-6",
                            {(info.render)()}
                        }
                    }
                }
                None => {
                    rsx! {
                        h1 { class: "mb-4 font-display text-3xl font-extrabold uppercase text-foreground md:text-4xl",
                            "Lab not found"
                        }
                        p { class: "mb-6 text-muted-foreground",
                            "No lab with that slug exists."
                        }
                        Link {
                            to: Route::LabsList {},
                            class: "font-mono text-sm text-accent hover:underline",
                            "\u{2190} Back to Labs"
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
