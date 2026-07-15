use std::sync::LazyLock;

use crate::blog::content::{self, Block};

include!(concat!(env!("OUT_DIR"), "/project_posts.rs"));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectFrontmatter {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub github: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectPost {
    pub slug: String,
    pub frontmatter: ProjectFrontmatter,
    pub body: Vec<Block>,
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "-")
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
    let content = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))?;
    let end = content.find("\n---")?;
    let (frontmatter, rest) = content.split_at(end);
    let body = rest
        .strip_prefix("\n---")?
        .strip_prefix('\n')
        .unwrap_or(rest);
    Some((frontmatter, body))
}

fn parse_project_post(filename: &str, raw: &str) -> Option<ProjectPost> {
    let (fm_str, body_md) = split_frontmatter(raw)?;
    let frontmatter: ProjectFrontmatter = serde_yaml::from_str(fm_str).ok()?;

    let body = match markdown::to_mdast(body_md, &markdown::ParseOptions::gfm()) {
        Ok(root) => content::from_mdast(&root),
        Err(_) => Vec::new(),
    };

    Some(ProjectPost {
        slug: slugify(filename),
        frontmatter,
        body,
    })
}

pub static PROJECTS: LazyLock<Vec<ProjectPost>> = LazyLock::new(|| {
    let mut projects: Vec<ProjectPost> = Vec::new();

    for (name, raw) in MARKDOWN_FILES {
        if let Some(post) = parse_project_post(name, raw) {
            projects.push(post);
        }
    }

    projects.sort_by(|a, b| b.frontmatter.timestamp.cmp(&a.frontmatter.timestamp));
    projects
});

pub fn project_by_slug(slug: &str) -> Option<&'static ProjectPost> {
    PROJECTS.iter().find(|p| p.slug == slug)
}

pub fn all_projects() -> &'static Vec<ProjectPost> {
    &PROJECTS
}
