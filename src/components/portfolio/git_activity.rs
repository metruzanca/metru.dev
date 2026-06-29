use dioxus::prelude::*;

use crate::github;
use super::section_heading::PortfolioSectionHeading;

const LEVEL_CLASSES: &[&str] = &[
    "bg-muted/40",
    "bg-primary/30",
    "bg-primary/55",
    "bg-primary/80",
    "bg-primary shadow-glow-pink",
];

fn month_labels(cells: &[github::ContributionCell]) -> Vec<String> {
    if cells.is_empty() {
        return vec![];
    }
    let n = cells.len();
    let step = n / 5;
    let mut labels = Vec::with_capacity(6);
    for i in (0..n).step_by(step.max(1)) {
        let d = cells[i].date;
        if d.len() >= 3 {
            labels.push(d[..3].to_string());
        }
        if labels.len() == 5 { break; }
    }
    // Always include the last month
    if let Some(last) = cells.last() {
        let d = last.date;
        if d.len() >= 3 {
            let last_label = d[..3].to_string();
            if labels.last().map_or(true, |l| l != &last_label) {
                labels.push(last_label);
            }
        }
    }
    labels
}

#[component]
pub fn GitActivity() -> Element {
    let total = github::contribution_total();
    let cells = github::contribution_cells();
    let months = month_labels(cells);

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
                        "@metruzanca on github"
                    }
                }

                div { class: "mb-1.5 flex justify-between font-mono text-[0.65rem] text-muted-foreground",
                    for month in &months {
                        span { "{month}" }
                    }
                }

                div {
                    class: "grid gap-[2px]",
                    role: "img",
                    aria_label: "GitHub contribution graph showing {total} contributions over the last year",
                    style: "grid-template-columns: repeat(52, 1fr); grid-template-rows: repeat(7, 1fr); grid-auto-flow: column;",
                    for cell in cells.iter() {
                        span {
                            class: "aspect-square rounded-[2px] {LEVEL_CLASSES[cell.level as usize]}",
                            title: if cell.count > 0 {
                                format!("{} contribution{} on {}", cell.count, if cell.count == 1 { "" } else { "s" }, cell.date)
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
