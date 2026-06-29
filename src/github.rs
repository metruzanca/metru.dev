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
    pub committed_date: Option<&'static str>,
}

pub struct ContributionCell {
    pub level: u8,
    pub count: u32,
    pub date: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/github_repos.rs"));

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
