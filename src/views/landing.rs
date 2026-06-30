use dioxus::prelude::*;

use crate::components::portfolio::{
    GitActivity, NowPlaying, NowPlayingStats, PortfolioContact, PortfolioHero, PortfolioProjects,
    PortfolioSectionHeading, PortfolioWriting,
};

#[component]
pub fn Landing() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl",
            PortfolioHero {}
            section { id: "music", class: "px-4 py-12",
                PortfolioSectionHeading {
                    index: "00".to_string(),
                    label: "music".to_string(),
                    action: None,
                }
                p { class: "mb-6 font-mono text-sm text-muted-foreground",
                    "I love listening to music while I work. Here's what I'm currently listening to:"
                }
                div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                    NowPlaying {}
                    NowPlayingStats {}
                }
            }
            PortfolioProjects {}
            PortfolioWriting {}
            GitActivity {}
            PortfolioContact {}

            footer { class: "border-t border-border px-4 py-8",
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
