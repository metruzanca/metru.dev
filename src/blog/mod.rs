use std::collections::BTreeMap;
use std::sync::LazyLock;

// Include auto-generated blog post content from build.rs
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BlogFrontmatter {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub canonical_url: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default = "default_true")]
    pub publish: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub frontmatter: BlogFrontmatter,
    /// Raw markdown body (after frontmatter)
    pub body_markdown: String,
}

fn extract_slug(canonical_url: &str) -> String {
    canonical_url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("unknown")
        .to_string()
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
    let content = raw.strip_prefix("---\n").or_else(|| raw.strip_prefix("---\r\n"))?;
    let end = content.find("\n---")?;
    let (frontmatter, rest) = content.split_at(end);
    let body = rest.strip_prefix("\n---")?.strip_prefix('\n').unwrap_or(rest);
    Some((frontmatter, body))
}

fn parse_blog_post(filename: &str, raw: &str) -> Option<BlogPost> {
    let (fm_str, body) = split_frontmatter(raw)?;
    let frontmatter: BlogFrontmatter = serde_yaml::from_str(fm_str).ok()?;
    let slug = if frontmatter.canonical_url.is_empty() {
        slugify(filename)
    } else {
        extract_slug(&frontmatter.canonical_url)
    };
    Some(BlogPost {
        slug,
        frontmatter,
        body_markdown: body.to_string(),
    })
}

pub static POSTS: LazyLock<Vec<BlogPost>> = LazyLock::new(|| {
    let mut posts: Vec<BlogPost> = ALL_BLOG_FILES
        .iter()
        .filter_map(|(name, content)| parse_blog_post(name, content))
        .collect();
    posts.sort_by(|a, b| b.frontmatter.timestamp.cmp(&a.frontmatter.timestamp));
    // Deduplicate by slug: keep the first (newest) occurrence
    let mut seen = std::collections::HashSet::new();
    posts.retain(|p| seen.insert(p.slug.clone()));
    posts
});

pub fn published_posts() -> Vec<&'static BlogPost> {
    POSTS.iter().filter(|p| p.frontmatter.publish).collect()
}

pub fn post_by_slug(slug: &str) -> Option<&'static BlogPost> {
    POSTS.iter().find(|p| p.slug == slug && p.frontmatter.publish)
}

pub fn all_tags() -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for post in POSTS.iter().filter(|p| p.frontmatter.publish) {
        for tag in &post.frontmatter.tags {
            *counts.entry(tag.clone()).or_default() += 1;
        }
    }
    counts.into_iter().collect()
}

pub fn render_markdown(markdown: &str) -> String {
    let html = markdown::to_html_with_options(markdown, &markdown::Options::gfm())
        .unwrap_or_else(|_| markdown.to_string());
    let html = rewrite_image_urls(&html);
    highlight_code_blocks(&html)
}

fn rewrite_image_urls(html: &str) -> String {
    let re = regex::Regex::new(r#"src="\./_assets/([^"]+)""#).unwrap();
    re.replace_all(html, r#"src="/assets/blog/$1""#).into_owned()
}

// -- Syntax highlighting ---------------------------------------------------

use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_newlines);
static THEME: LazyLock<syntect::highlighting::Theme> = LazyLock::new(|| {
    let ts = ThemeSet::load_defaults();
    ts.themes["base16-ocean.dark"].clone()
});

fn decode_html_entities(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
}

fn highlight_code_blocks(html: &str) -> String {
    let re = regex::Regex::new(
        r#"<pre><code class="language-([^"]*)">((?s).*?)</code></pre>"#,
    )
    .unwrap();

    re.replace_all(html, |caps: &regex::Captures| {
        let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let code = decode_html_entities(code);
        highlight_code(lang, &code)
            .unwrap_or_else(|_| format!("<pre><code>{code}</code></pre>"))
    })
    .into_owned()
}

fn highlight_code(lang: &str, code: &str) -> Result<String, syntect::Error> {
    let syntax = SYNTAXES
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

    let highlighted = highlighted_html_for_string(code, &SYNTAXES, syntax, &THEME)?;

    let lang_label = if lang.is_empty() {
        String::new()
    } else {
        format!("<div class=\"code-lang\">{lang}</div>")
    };

    Ok(format!(
        "<div class=\"code-block-wrapper\">{lang_label}{highlighted}</div>"
    ))
}
