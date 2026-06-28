use dioxus::prelude::*;

use super::section_heading::PortfolioSectionHeading;

const WEEKS: usize = 52;
const DAYS: usize = 7;

fn seeded(n: u32) -> f64 {
    let x = (n as f64 * 12.9898).sin() * 43758.5453;
    x - x.floor()
}

fn build_data() -> Vec<u8> {
    let mut cells = Vec::with_capacity(WEEKS * DAYS);
    for i in 0..(WEEKS * DAYS) {
        let r = seeded(i as u32 + 1);
        let level = if r > 0.92 {
            4
        } else if r > 0.8 {
            3
        } else if r > 0.6 {
            2
        } else if r > 0.38 {
            1
        } else {
            0
        };
        cells.push(level);
    }
    cells
}

const LEVEL_CLASSES: &[&str] = &[
    "bg-muted/40",
    "bg-primary/30",
    "bg-primary/55",
    "bg-primary/80",
    "bg-primary shadow-glow-pink",
];

const MONTHS: &[&str] = &["Jan", "Mar", "May", "Jul", "Sep", "Nov"];

#[component]
pub fn GitActivity() -> Element {
    let data = build_data();
    let total: u32 = data.iter().map(|&l| l as u32 * 3).sum();

    rsx! {
        section { id: "activity", class: "px-4 py-12",
            PortfolioSectionHeading {
                index: "03".to_string(),
                label: "activity".to_string(),
                action: None,
            }

            div { class: "rounded-xl border border-border bg-card p-5",
                div { class: "mb-4 flex flex-wrap items-baseline justify-between gap-2",
                    p { class: "text-sm text-foreground",
                        span { class: "font-display text-xl font-bold text-primary",
                            "{total}"
                        }
                        " "
                        span { class: "text-muted-foreground",
                            "contributions in the last year"
                        }
                    }
                    span { class: "font-mono text-xs text-muted-foreground",
                        "@sam on github"
                    }
                }

                div { class: "overflow-x-auto pb-1",
                    div { class: "min-w-[640px]",
                        div { class: "mb-1.5 flex justify-between px-0.5 font-mono text-[0.65rem] text-muted-foreground",
                            for month in MONTHS {
                                span { "{month}" }
                            }
                        }

                        div {
                            class: "grid grid-flow-col gap-1",
                            role: "img",
                            aria_label: "GitHub contribution graph showing {total} contributions over the last year",
                            style: "grid-template-rows: repeat(7, minmax(0, 1fr))",
                            for &level in &data {
                                span {
                                    class: "size-2.5 rounded-[2px] {LEVEL_CLASSES[level as usize]}",
                                }
                            }
                        }
                    }
                }

                div { class: "mt-4 flex items-center justify-end gap-2 font-mono text-[0.65rem] text-muted-foreground",
                    span { "less" }
                    for &cls in LEVEL_CLASSES {
                        span { class: "size-2.5 rounded-[2px] {cls}" }
                    }
                    span { "more" }
                }
            }
        }
    }
}
