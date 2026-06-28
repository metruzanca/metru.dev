use dioxus::prelude::*;

use crate::components::ui::icons::MapPinIcon;

// const STATS: &[(&str, &str)] = &[
//     ("8+", "years shipping"),
//     ("40+", "projects built"),
//     ("1.2k", "github stars"),
//     ("12", "open source repos"),
// ];

#[component]
pub fn PortfolioHero() -> Element {
    rsx! {
        section { id: "top", class: "px-4 pb-12 pt-14 md:pt-20",
            // div { class: "flex items-center gap-2 font-mono text-xs text-accent",
            //     span { class: "relative flex size-2",
            //         span { class: "absolute inline-flex size-full animate-ping rounded-full bg-accent opacity-60" }
            //         CircleIcon { class: "size-2 fill-accent text-accent" }
            //     }
            //     "available for new work"
            // }

            h1 { class: "mt-5 font-display text-4xl font-extrabold uppercase leading-[1.05] tracking-tight text-balance text-foreground md:text-6xl",
                "Hi, I'm "
                span { class: "text-primary text-glow-pink", "Sam" }
                "!"
            }

            p { class: "mt-5 max-w-xl text-pretty leading-relaxed text-muted-foreground md:text-lg",
                "A full-stack engineer who builds fast, reliable products for the open web. \
                 Years of React and TypeScript, now chasing performance with Go and Rust. \
                 I value simplicity, minimalism, and shipping things that work."
            }

            div { class: "mt-5 flex flex-wrap items-center gap-4 font-mono text-xs text-muted-foreground",
                span { class: "inline-flex items-center gap-1.5",
                    MapPinIcon { class: "size-3.5 text-primary" }
                    "Jersey City, NJ"
                }
                span { class: "inline-flex items-center gap-1.5",
                    span { class: "size-1.5 rounded-full bg-accent shadow-glow-cyan" }
                    "React \u{b7} TypeScript \u{b7} Go \u{b7} Rust"
                }
            }

            // dl { class: "mt-10 grid grid-cols-2 gap-px overflow-hidden rounded-xl border border-border bg-border md:grid-cols-4",
            //     for (value, label) in STATS {
            //         div { class: "bg-card p-5",
            //             dt { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
            //                 "{label}"
            //             }
            //             dd { class: "mt-2 font-display text-3xl font-bold text-foreground",
            //                 "{value}"
            //             }
            //         }
            //     }
            // }
        }
    }
}
