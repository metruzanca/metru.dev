use dioxus::prelude::*;

use crate::github;

const TARGET_REPOS: &[&str] = &["squeal", "atcrab", "metru.dev"];

const LEVEL_CLASSES: &[&str] = &[
    "bg-muted/15",
    "bg-primary/20",
    "bg-primary/40",
    "bg-primary/65",
    "bg-primary shadow-glow-pink",
];

fn iso_to_week_number(iso: &str) -> Option<u32> {
    let date = if iso.len() >= 10 {
        &iso[..10]
    } else {
        return None;
    };
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year: i64 = parts[0].parse().ok()?;
    let month: i64 = parts[1].parse().ok()?;
    let day: i64 = parts[2].parse().ok()?;
    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut doy: i64 = 0;
    for m in 0..(month - 1) {
        doy += month_days[m as usize];
    }
    doy += day;
    let days_since_epoch = (year - 1970) * 365 + (year - 1969) / 4 + doy - 1;
    let unix_ts = days_since_epoch * 86400;
    let monday_epoch: i64 = 345600;
    let week_seconds: i64 = 604800;
    if unix_ts < monday_epoch {
        return Some(0);
    }
    Some(((unix_ts - monday_epoch) / week_seconds) as u32)
}

fn commit_level(count: u32, max_count: u32) -> usize {
    if count == 0 {
        return 0;
    }
    if max_count == 0 {
        return 1;
    }
    let ratio = count as f64 / max_count as f64;
    if ratio > 0.75 {
        4
    } else if ratio > 0.5 {
        3
    } else if ratio > 0.25 {
        2
    } else {
        1
    }
}

struct RepoRowData {
    name: &'static str,
    created_week: Option<usize>,
    committed_week: Option<usize>,
    week_data: std::collections::HashMap<usize, u32>,
}

#[component]
pub fn DsProjectGantt() -> Element {
    let repos = github::all_repos();
    let timelines_raw = github::repo_timelines_json();
    let timelines = github::parse_repo_timelines(timelines_raw);

    let mut target_repos: Vec<&github::GithubRepo> = repos
        .iter()
        .filter(|r| TARGET_REPOS.contains(&r.name))
        .collect();
    target_repos.sort_by_key(|r| r.name);

    let mut week_map: std::collections::HashMap<String, std::collections::HashMap<usize, u32>> =
        std::collections::HashMap::new();
    let mut max_commits_in_week: u32 = 0;
    for tl in &timelines {
        let mut inner = std::collections::HashMap::new();
        for &(week, count) in &tl.weeks {
            inner.insert(week as usize, count);
            if count > max_commits_in_week {
                max_commits_in_week = count;
            }
        }
        week_map.insert(tl.name.clone(), inner);
    }

    let mut min_week: usize = usize::MAX;
    let mut max_week: usize = 0;
    for repo in &target_repos {
        if let Some(w) = repo.created_at.and_then(iso_to_week_number) {
            let w = w as usize;
            if w < min_week {
                min_week = w
            }
        }
        if let Some(w) = repo.committed_date.and_then(iso_to_week_number) {
            let w = w as usize;
            if w > max_week {
                max_week = w
            }
        }
    }

    if min_week == usize::MAX || max_week == 0 || target_repos.is_empty() {
        return rsx! {
            div { class: "py-8 text-center font-mono text-xs text-muted-foreground",
                "No timeline data available."
            }
        };
    }

    let mut year_labels: Vec<(usize, i32)> = Vec::new();
    for year in 1970..2100 {
        let iso = format!("{}-01-01", year);
        if let Some(w) = iso_to_week_number(&iso) {
            let w = w as usize;
            if w >= min_week && w <= max_week {
                year_labels.push((w - min_week, year));
            }
        }
        if year > 1970 + (max_week - min_week + 1) as i32 / 52 + 2 {
            break;
        }
    }

    let total_weeks = max_week - min_week + 1;
    let cell_size: usize = 8;
    let name_col_width: usize = 140;

    let mut rows: Vec<RepoRowData> = Vec::new();
    for repo in &target_repos {
        rows.push(RepoRowData {
            name: repo.name,
            created_week: repo.created_at.and_then(iso_to_week_number).map(|w| w as usize),
            committed_week: repo.committed_date.and_then(iso_to_week_number).map(|w| w as usize),
            week_data: week_map.get(repo.name).cloned().unwrap_or_default(),
        });
    }

    let total_width = name_col_width + total_weeks * cell_size;
    let years = year_labels;

    rsx! {
        section { class: "flex flex-col gap-5",
            div { class: "flex flex-col gap-2",
                span { class: "font-mono text-xs uppercase tracking-widest text-primary",
                    "Components"
                }
                h2 { class: "font-display text-2xl font-bold uppercase text-foreground",
                    "Project Timeline"
                }
                p { class: "max-w-lg text-sm leading-relaxed text-muted-foreground",
                    "Commit activity over time for select projects. Darker segments mean more commits per week."
                }
            }

            div { class: "overflow-x-auto pb-2",
                div {
                    class: "flex flex-col gap-2",
                    style: "width: {total_width}px",

                    div { class: "flex items-end h-5",
                        div { class: "shrink-0", style: "width: {name_col_width}px" }
                        div {
                            class: "flex relative",
                            style: "width: {total_weeks * cell_size}px; height: 0",
                            for (offset, year) in &years {
                                span {
                                    key: "{year}",
                                    class: "absolute font-mono text-[0.65rem] text-muted-foreground",
                                    style: "left: {offset * cell_size}px; top: -2px",
                                    "{year}"
                                }
                            }
                        }
                    }

                    for row in rows {
                        {render_row(
                            row,
                            min_week,
                            total_weeks,
                            cell_size,
                            name_col_width,
                            max_commits_in_week,
                        )}
                    }

                    div { class: "flex items-center gap-3 mt-1",
                        div { class: "shrink-0", style: "width: {name_col_width}px" }
                        div { class: "flex items-center gap-1.5 font-mono text-[0.65rem] text-muted-foreground",
                            span { "Less" }
                            for level in 0..5usize {
                                span { class: "size-3 rounded-sm {LEVEL_CLASSES[level]}" }
                            }
                            span { "More" }
                        }
                    }
                }
            }
        }
    }
}

struct SegmentStyle {
    cls: &'static str,
    title: String,
}

fn compute_segments(
    row: &RepoRowData,
    min_week: usize,
    total_weeks: usize,
    max_commits: u32,
) -> Vec<SegmentStyle> {
    let mut segments = Vec::with_capacity(total_weeks);
    for week_offset in 0..total_weeks {
        let global_week = min_week + week_offset;
        let is_active = match (row.created_week, row.committed_week) {
            (Some(cw), Some(lw)) => global_week >= cw && global_week <= lw,
            _ => false,
        };
        let count = row.week_data.get(&global_week).copied().unwrap_or(0);
        let level = if is_active {
            commit_level(count, max_commits)
        } else {
            0
        };
        let cls = if is_active && count == 0 {
            "bg-muted/15"
        } else {
            LEVEL_CLASSES[level]
        };
        let title = if count > 0 {
            format!("{count} commit{}", if count == 1 { "" } else { "s" })
        } else if is_active {
            "No commits".to_string()
        } else {
            String::new()
        };
        segments.push(SegmentStyle { cls, title });
    }
    segments
}

fn render_row(
    row: RepoRowData,
    min_week: usize,
    total_weeks: usize,
    cell_size: usize,
    name_col_width: usize,
    max_commits: u32,
) -> Element {
    let segments = compute_segments(&row, min_week, total_weeks, max_commits);

    rsx! {
        div { class: "flex items-center gap-0",
            a {
                class: "shrink-0 truncate font-mono text-xs text-foreground transition-colors hover:text-primary pr-3 text-right",
                style: "width: {name_col_width}px",
                href: "https://github.com/metruzanca/{row.name}",
                target: "_blank",
                rel: "noreferrer",
                "{row.name}"
            }
            div {
                class: "flex",
                style: "width: {total_weeks * cell_size}px",
                for (week_offset, seg) in segments.iter().enumerate() {
                    span {
                        key: "{row.name}-{week_offset}",
                        class: "h-5 {seg.cls}",
                        style: "width: {cell_size}px",
                        title: "{seg.title}",
                    }
                }
            }
        }
    }
}
