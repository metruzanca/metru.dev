use dioxus::prelude::*;

use ui::icons::GitBranchIcon;
use crate::github::{self, GithubRepo};
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
pub fn ProjectsList() -> Element {
    let repos = github::all_repos();
    let case_studies = projects::all_projects();

    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pb-12 pt-14 md:pt-20",
            h1 { class: "mb-8 font-display text-3xl font-extrabold uppercase text-foreground md:text-4xl",
                "Projects"
            }

            if !case_studies.is_empty() {
                h2 { class: "mb-4 font-display text-xl font-semibold uppercase text-accent md:text-2xl",
                    "Case Studies"
                }
                div { class: "mb-12 grid grid-cols-1 gap-4",
                    for project in case_studies.iter() {
                        CaseStudyCard { project }
                    }
                }
            }

            h2 { class: "mb-4 font-display text-xl font-semibold uppercase text-primary md:text-2xl",
                "Open Source"
            }
            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                for repo in repos {
                    ProjectCard { repo: *repo }
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
                if !fm.github.is_empty() {
                    span { class: "hidden shrink-0 font-mono text-xs text-muted-foreground sm:block",
                        "source \u{2192}"
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

fn tag_accent(index: usize) -> &'static str {
    if index % 2 == 0 { "primary" } else { "cyan" }
}

fn tag_classes(accent: &str) -> &'static str {
    match accent {
        "cyan" => "rounded-md border px-2 py-0.5 font-mono text-[0.7rem] border-accent/40 text-accent",
        _ => "rounded-md border px-2 py-0.5 font-mono text-[0.7rem] border-primary/40 text-primary",
    }
}

fn format_commit_date(iso: &str) -> String {
    if iso.len() < 10 { return String::new(); }
    let month = match &iso[5..7] {
        "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
        "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
        "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
        _ => "",
    };
    let year = &iso[0..4];
    format!("{month} {year}")
}

#[component]
fn ProjectCard(repo: GithubRepo) -> Element {
    let star_count = if repo.stars >= 1000 {
        format!("{:.1}k", repo.stars as f64 / 1000.0)
    } else {
        repo.stars.to_string()
    };

    let tags: Vec<&str> = repo.topics.iter().take(4).copied().collect();

    rsx! {
        a {
            class: "group relative flex flex-col rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
            href: "{repo.url}",
            target: "_blank",
            rel: "noreferrer",
            if repo.language.is_some() || repo.stars > 0 || repo.commit_count > 0 {
                div { class: "flex items-center justify-between gap-3 mb-2",
                    if let Some(lang) = repo.language {
                        div { class: "flex items-center gap-1.5",
                            if let Some(color) = repo.language_color {
                                span {
                                    class: "inline-block size-2 rounded-full",
                                    style: "background-color: {color}",
                                }
                            }
                            span { class: "font-mono text-[0.7rem] uppercase tracking-wider text-muted-foreground",
                                "{lang}"
                            }
                        }
                    } else {
                        span {}
                    }
                    span { class: "inline-flex items-center gap-2 font-mono text-xs text-muted-foreground",
                        if repo.stars > 0 {
                            span { class: "inline-flex items-center gap-1",
                                span { class: "text-primary", "\u{2605}" }
                                "{star_count}"
                            }
                        }
                        if repo.commit_count > 0 {
                            span { class: "inline-flex items-center gap-1",
                                GitBranchIcon { class: "size-3.5" }
                                "{repo.commit_count}"
                            }
                        }
                    }
                }
            }
            div { class: "flex items-start justify-between gap-3",
                h3 { class: "font-display text-lg font-bold uppercase tracking-tight text-foreground transition-colors group-hover:underline",
                    "{repo.name}"
                }
            }

            if !repo.description.is_empty() {
                p { class: "mt-2 flex-1 text-sm leading-relaxed text-muted-foreground",
                    "{repo.description}"
                }
            }

            div { class: "mt-4 flex items-center justify-between gap-3",
                div { class: "flex flex-wrap gap-1.5",
                    for (i, tag) in tags.iter().enumerate() {
                        span {
                            class: "{tag_classes(tag_accent(i))}",
                            "{tag}"
                        }
                    }
                }
                if let Some(date) = repo.committed_date {
                    span { class: "font-mono text-[0.65rem] text-muted-foreground/60 whitespace-nowrap",
                        "{format_commit_date(date)}"
                    }
                }
            }
        }
    }
}
