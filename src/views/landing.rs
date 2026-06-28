use dioxus::prelude::*;

use crate::components::portfolio::{
    PortfolioHero, PortfolioProjects, PortfolioWriting, GitActivity, PortfolioContact,
};

#[component]
pub fn Landing() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl",
            PortfolioHero {}
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
                    p { class: "font-mono text-xs text-muted-foreground",
                        "designed in the neon"
                    }
                }
            }
        }
    }
}
