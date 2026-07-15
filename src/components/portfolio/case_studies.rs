use dioxus::prelude::*;

use crate::projects;
use crate::Route;

fn format_date(iso: &str) -> String {
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

#[component]
pub fn PortfolioCaseStudies() -> Element {
    let all = projects::all_projects();
    let recent = all.iter().take(3);

    if all.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { id: "case-studies", class: "px-4 py-12",
            div { class: "mb-6 flex items-center justify-end border-b border-border pb-2",
                a {
                    class: "font-mono text-xs text-muted-foreground transition-colors hover:text-accent hover:text-glow-cyan",
                    href: "/projects",
                    "view all \u{2192}"
                }
            }

            div { class: "grid grid-cols-1 gap-4",
                for project in recent {
                    CaseStudyCard { project }
                }
            }
        }
    }
}

#[component]
fn CaseStudyCard(project: &'static projects::ProjectPost) -> Element {
    let fm = &project.frontmatter;
    let date = format_date(&fm.timestamp);

    rsx! {
        Link {
            to: Route::ProjectCaseStudy { slug: project.slug.clone() },
            class: "group block rounded-xl border border-border bg-card p-5 transition-colors hover:border-accent/50",
            div { class: "flex items-baseline gap-4",
                time { class: "shrink-0 font-mono text-xs text-muted-foreground",
                    "{date}"
                }
                div { class: "flex-1 min-w-0",
                    h3 { class: "truncate font-display text-lg font-bold uppercase tracking-tight text-foreground transition-colors group-hover:text-accent",
                        "{fm.title}"
                    }
                    if let Some(company) = &fm.company {
                        p { class: "mt-0.5 font-mono text-xs text-accent",
                            "{company}"
                        }
                    }
                }
            }
            if !fm.description.is_empty() {
                p { class: "mt-2 text-sm leading-relaxed text-muted-foreground",
                    "{fm.description}"
                }
            }
            if !fm.tags.is_empty() {
                div { class: "mt-3 flex flex-wrap gap-1.5",
                    for tag in &fm.tags {
                        span { class: "rounded-md border border-border/60 px-2 py-0.5 font-mono text-[0.65rem] text-muted-foreground",
                            "#{tag}"
                        }
                    }
                }
            }
        }
    }
}
