use dioxus::prelude::*;

use crate::components::ui::icons::MapPinIcon;

#[component]
pub fn PortfolioHero() -> Element {
    rsx! {
        section { id: "top", class: "px-4 pb-12 pt-14 md:pt-20",
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
            a {
                class: "mt-5 inline-flex items-center gap-2 font-mono text-sm rounded-lg border border-accent/30 bg-accent/5 px-4 py-2 text-accent transition-all hover:border-accent/60 hover:bg-accent/10 hover:text-glow-cyan",
                href: "/how-i-work",
                "Read how I work"
                span { class: "text-accent/60", "\u{2192}" }
            }
        }
    }
}
