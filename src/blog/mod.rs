use std::collections::BTreeMap;
use std::sync::LazyLock;

// Include auto-generated blog post content from build.rs
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

pub fn published_posts_owned() -> Vec<BlogPost> {
    published_posts().into_iter().cloned().collect()
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

// ── Server-only: live ATProto cache with hourly background refresh ────

#[cfg(not(target_arch = "wasm32"))]
mod live {
    use std::sync::{OnceLock, RwLock};
    use std::time::Duration;

    use super::*;

    struct MergedCache {
        snapshot: Vec<BlogPost>,
        live: Vec<BlogPost>,
    }

    static CACHE: OnceLock<RwLock<MergedCache>> = OnceLock::new();

    fn init_cache() -> &'static RwLock<MergedCache> {
        CACHE.get_or_init(|| {
            RwLock::new(MergedCache {
                snapshot: POSTS.iter().cloned().collect(),
                live: Vec::new(),
            })
        })
    }

    pub async fn fetch_live_posts() -> Vec<BlogPost> {
        let repo = match atcrab::Repo::new("metru.dev").await {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        let records = match repo
            .fetch_all_collection::<atcrab::lexicons::Document>()
            .await
        {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        records
            .into_iter()
            .filter_map(|record| document_to_post(record.value))
            .collect()
    }

    fn document_to_post(doc: atcrab::lexicons::Document) -> Option<BlogPost> {
        let slug = slugify(&doc.title);
        let body = blocks_to_markdown(&doc.content, &doc.text_content);
        Some(BlogPost {
            slug,
            frontmatter: BlogFrontmatter {
                title: doc.title,
                description: doc.description.unwrap_or_default(),
                canonical_url: String::new(),
                timestamp: doc.published_at,
                publish: true,
                tags: doc.tags.unwrap_or_default(),
            },
            body_markdown: body,
        })
    }

    pub fn get_merged_posts() -> Vec<BlogPost> {
        let cache = init_cache();
        let guard = cache.read().unwrap();
        let mut all = guard.snapshot.clone();
        for live in &guard.live {
            if let Some(pos) = all.iter().position(|p| p.slug == live.slug) {
                all[pos] = live.clone();
            } else {
                all.push(live.clone());
            }
        }
        all.sort_by(|a, b| b.frontmatter.timestamp.cmp(&a.frontmatter.timestamp));
        all
    }

    pub fn start_background_refresh() {
        // Pre-warm the cache on the calling thread before spawning
        init_cache();

        std::thread::spawn(|| {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create ATProto background runtime: {e}");
                    return;
                }
            };

            // Brief delay so the server can finish starting up
            std::thread::sleep(Duration::from_secs(10));

            loop {
                let live = rt.block_on(fetch_live_posts());
                if let Some(cache) = CACHE.get() {
                    if let Ok(mut guard) = cache.write() {
                        guard.live = live;
                    }
                }
                std::thread::sleep(Duration::from_secs(3600));
            }
        });
    }

    // ── ATProto content block → markdown conversion ───────────────────

    fn blocks_to_markdown(
        content: &Option<serde_json::Value>,
        text_content: &Option<String>,
    ) -> String {
        let content = match content {
            Some(c) => c,
            None => return text_content.clone().unwrap_or_default(),
        };

        let pages = match content.get("pages").and_then(|p| p.as_array()) {
            Some(p) => p,
            None => return text_content.clone().unwrap_or_default(),
        };

        let mut markdown = String::new();
        for page in pages {
            let blocks = match page.get("blocks").and_then(|b| b.as_array()) {
                Some(b) => b,
                None => continue,
            };

            for block_value in blocks {
                let block = match block_value.get("block") {
                    Some(b) => b,
                    None => continue,
                };

                let block_type = block
                    .get("$type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("");
                let plaintext = block
                    .get("plaintext")
                    .and_then(|t| t.as_str())
                    .unwrap_or("");

                match block_type {
                    t if t.ends_with(".blocks.header") => {
                        let level = block.get("level").and_then(|l| l.as_u64()).unwrap_or(1);
                        let prefix = "#".repeat(level as usize);
                        let text = apply_facets_to_markdown(plaintext, block.get("facets"));
                        markdown.push_str(&format!("{prefix} {text}\n\n"));
                    }
                    t if t.ends_with(".blocks.text") => {
                        let text = apply_facets_to_markdown(plaintext, block.get("facets"));
                        markdown.push_str(&format!("{text}\n\n"));
                    }
                    t if t.ends_with(".blocks.bullet") || t.ends_with(".blocks.list_item") => {
                        let text = apply_facets_to_markdown(plaintext, block.get("facets"));
                        let indent = block.get("indent").and_then(|i| i.as_u64()).unwrap_or(0);
                        let prefix = "  ".repeat(indent as usize);
                        markdown.push_str(&format!("{prefix}- {text}\n\n"));
                    }
                    t if t.ends_with(".blocks.blockquote") || t.ends_with(".blocks.quote") => {
                        let text = apply_facets_to_markdown(plaintext, block.get("facets"));
                        for line in text.lines() {
                            markdown.push_str(&format!("> {line}\n"));
                        }
                        markdown.push_str("\n");
                    }
                    t if t.ends_with(".blocks.horizontalRule") => {
                        markdown.push_str("---\n\n");
                    }
                    t if t.ends_with(".blocks.unorderedList") || t.ends_with(".blocks.orderedList") => {
                        let numbered = t.ends_with(".blocks.orderedList");
                        if let Some(children) = block.get("children").and_then(|c| c.as_array()) {
                            for (i, child) in children.iter().enumerate() {
                                if let Some(content) = child.get("content") {
                                    let item_text = content.get("plaintext").and_then(|t| t.as_str()).unwrap_or("");
                                    let text = apply_facets_to_markdown(item_text, content.get("facets"));
                                    let prefix = if numbered { format!("{}. ", i + 1) } else { "- ".to_string() };
                                    markdown.push_str(&format!("{prefix}{text}\n"));
                                }
                            }
                        }
                        markdown.push_str("\n");
                    }
                    _ => {
                        if !plaintext.is_empty() {
                            markdown.push_str(&format!("{plaintext}\n\n"));
                        }
                    }
                }
            }
        }

        if markdown.is_empty() {
            return text_content.clone().unwrap_or_default();
        }

        markdown
    }

    fn apply_facets_to_markdown(text: &str, facets: Option<&serde_json::Value>) -> String {
        let facets = match facets.and_then(|f| f.as_array()) {
            Some(f) if !f.is_empty() => f,
            _ => return text.to_string(),
        };

        let mut sorted: Vec<&serde_json::Value> = facets.iter().collect();
        sorted.sort_by_key(|f| {
            f.get("index")
                .and_then(|i| i.get("byteStart"))
                .and_then(|b| b.as_u64())
                .unwrap_or(0)
        });

        let mut output = String::new();
        let mut last_end: usize = 0;
        let text_bytes = text.as_bytes();
        let text_len = text.len();

        for facet in &sorted {
            let index = match facet.get("index") {
                Some(i) => i,
                None => continue,
            };
            let start = index
                .get("byteStart")
                .and_then(|b| b.as_u64())
                .unwrap_or(0) as usize;
            let end = index
                .get("byteEnd")
                .and_then(|b| b.as_u64())
                .unwrap_or(0) as usize;

            if start >= text_len || end > text_len || start >= end {
                continue;
            }

            let uri = facet
                .get("features")
                .and_then(|f| f.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .find_map(|feat| feat.get("uri").and_then(|u| u.as_str()))
                });

            if start > last_end {
                output.push_str(&text[last_end..start]);
            }

            let segment = std::str::from_utf8(&text_bytes[start..end]).unwrap_or("");
            if let Some(uri) = uri {
                output.push_str(&format!("[{segment}]({uri})"));
            } else {
                output.push_str(segment);
            }

            last_end = end;
        }

        if last_end < text_len {
            output.push_str(&text[last_end..]);
        }

        output
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use live::*;
