#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GithubRepo {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub homepage: Option<&'static str>,
    pub stars: u32,
    pub commit_count: u32,
    pub language: Option<&'static str>,
    pub language_color: Option<&'static str>,
    pub topics: &'static [&'static str],
    pub created_at: Option<&'static str>,
    pub committed_date: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct RepoTimeline {
    pub name: String,
    pub weeks: Vec<(u32, u32)>, // (week_number_since_epoch, commit_count)
}

pub struct ContributionCell {
    pub level: u8,
    pub count: u32,
    pub date: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/github_repos.rs"));
include!(concat!(env!("OUT_DIR"), "/repo_timelines.rs"));

pub fn pinned_repos() -> &'static [GithubRepo] {
    PINNED_REPOS
}

pub fn all_repos() -> &'static [GithubRepo] {
    ALL_REPOS
}

pub fn contribution_total() -> u32 {
    CONTRIBUTION_TOTAL
}

pub fn contribution_cells() -> &'static [ContributionCell] {
    CONTRIBUTION_CELLS
}

pub fn repo_timelines_json() -> &'static str {
    REPO_TIMELINES_JSON
}

pub fn parse_repo_timelines(json: &str) -> Vec<RepoTimeline> {
    use std::collections::BTreeMap;

    #[derive(serde::Deserialize)]
    struct RawTimeline {
        weeks: Vec<(u32, u32)>,
    }

    let map: BTreeMap<String, RawTimeline> = match serde_json::from_str(json) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    map.into_iter()
        .map(|(name, raw)| RepoTimeline {
            name,
            weeks: raw.weeks,
        })
        .collect()
}
