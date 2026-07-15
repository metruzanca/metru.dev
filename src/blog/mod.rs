pub mod content;

use std::collections::BTreeMap;
use std::sync::LazyLock;

#[allow(unused_imports)]
pub use content::{render_blocks, Block, Inline};

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
    /// Structured content blocks — the canonical body representation.
    /// Rendered to HTML via `content::render_blocks`.
    pub body: Vec<Block>,
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

fn parse_blog_post(filename: &str, raw: &str) -> Option<BlogPost> {
    let (fm_str, body_md) = split_frontmatter(raw)?;
    let frontmatter: BlogFrontmatter = serde_yaml::from_str(fm_str).ok()?;
    let slug = if frontmatter.canonical_url.is_empty() {
        slugify(filename)
    } else {
        extract_slug(&frontmatter.canonical_url)
    };

    let body = match markdown::to_mdast(body_md, &markdown::ParseOptions::gfm()) {
        Ok(root) => content::from_mdast(&root),
        Err(_) => Vec::new(),
    };

    Some(BlogPost {
        slug,
        frontmatter,
        body,
    })
}

pub static POSTS: LazyLock<Vec<BlogPost>> = LazyLock::new(|| {
    let mut posts: Vec<BlogPost> = Vec::new();

    // Parse markdown source files
    for (name, content) in MARKDOWN_FILES {
        if let Some(post) = parse_blog_post(name, content) {
            posts.push(post);
        }
    }

    // Deserialize pre-built ATProto posts
    if let Ok(atproto_posts) = serde_json::from_str::<Vec<BlogPost>>(ATPROTO_POSTS_JSON) {
        posts.extend(atproto_posts);
    }

    posts.sort_by(|a, b| b.frontmatter.timestamp.cmp(&a.frontmatter.timestamp));
    let mut seen = std::collections::HashSet::new();
    posts.retain(|p| seen.insert(p.slug.clone()));
    posts
});

pub fn published_posts() -> Vec<&'static BlogPost> {
    POSTS.iter().filter(|p| p.frontmatter.publish).collect()
}

pub fn post_by_slug(slug: &str) -> Option<&'static BlogPost> {
    POSTS
        .iter()
        .find(|p| p.slug == slug && p.frontmatter.publish)
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

// ── Server-only: live ATProto cache with hourly background refresh ────

#[cfg(not(target_arch = "wasm32"))]
mod live {
    use std::sync::{OnceLock, RwLock};
    use std::time::Duration;

    use crate::blog::content::{Block, Inline};

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

        let did = repo.did.clone();
        let pds = repo.pds.clone();

        let records = match repo
            .fetch_all_collection::<atcrab::lexicons::Document>()
            .await
        {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        records
            .into_iter()
            .filter_map(|record| document_to_post(record.value, &did, &pds))
            .collect()
    }

    fn document_to_post(
        doc: atcrab::lexicons::Document,
        did: &str,
        pds: &str,
    ) -> Option<BlogPost> {
        let mut body = blocks_to_body(&doc.content, &doc.text_content);

        if let Some(ref cover) = doc.cover_image {
            let blob_url = format!(
                "{}/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                pds.trim_end_matches('/'),
                did,
                cover.blob_ref.link,
            );
            body.insert(
                0,
                Block::Paragraph(vec![Inline::Image {
                    alt: doc.title.clone(),
                    src: blob_url,
                }]),
            );
        }

        Some(BlogPost {
            slug: slugify(&doc.title),
            frontmatter: BlogFrontmatter {
                title: doc.title,
                description: doc.description.unwrap_or_default(),
                canonical_url: String::new(),
                timestamp: doc.published_at,
                publish: true,
                tags: doc.tags.unwrap_or_default(),
            },
            body,
        })
    }

    pub fn get_merged_posts() -> Vec<BlogPost> {
        let cache = init_cache();
        let guard = cache.read().unwrap();
        let mut all: Vec<BlogPost> = guard
            .snapshot
            .iter()
            .filter(|p| p.frontmatter.publish)
            .cloned()
            .collect();
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
        init_cache();

        std::thread::spawn(|| {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create ATProto background runtime: {e}");
                    return;
                }
            };

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

    // ── ATProto content block → Block conversion ───────────────────

    fn blocks_to_body(
        content: &Option<serde_json::Value>,
        text_content: &Option<String>,
    ) -> Vec<Block> {
        let content = match content {
            Some(c) => c,
            None => return Vec::new(),
        };

        let pages = match content.get("pages").and_then(|p| p.as_array()) {
            Some(p) => p,
            None => return Vec::new(),
        };

        let mut blocks = Vec::new();
        for page in pages {
            let page_blocks = match page.get("blocks").and_then(|b| b.as_array()) {
                Some(b) => b,
                None => continue,
            };

            for block_value in page_blocks {
                let block = match block_value.get("block") {
                    Some(b) => b,
                    None => continue,
                };

                if let Some(converted) = atproto_block(block) {
                    blocks.push(converted);
                }
            }
        }

        if blocks.is_empty() {
            if let Some(text) = text_content {
                blocks.push(Block::Paragraph(vec![Inline::Text(text.clone())]));
            }
        }

        blocks
    }

    fn atproto_block(block: &serde_json::Value) -> Option<Block> {
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
                let level = block.get("level").and_then(|l| l.as_u64()).unwrap_or(1) as u8;
                let inlines = apply_facets_to_inlines(plaintext, block.get("facets"));
                Some(Block::Heading {
                    level,
                    children: inlines,
                })
            }
            t if t.ends_with(".blocks.text") => {
                let inlines = apply_facets_to_inlines(plaintext, block.get("facets"));
                Some(Block::Paragraph(inlines))
            }
            t if t.ends_with(".blocks.bullet") || t.ends_with(".blocks.list_item") => {
                let inlines = apply_facets_to_inlines(plaintext, block.get("facets"));
                // Each bullet/item becomes a single-item list for simplicity
                Some(Block::UnorderedList(vec![inlines]))
            }
            t if t.ends_with(".blocks.blockquote") || t.ends_with(".blocks.quote") => {
                let inlines = apply_facets_to_inlines(plaintext, block.get("facets"));
                Some(Block::Blockquote(vec![Block::Paragraph(inlines)]))
            }
            t if t.ends_with(".blocks.horizontalRule") => Some(Block::ThematicBreak),
            t if t.ends_with(".blocks.unorderedList")
                || t.ends_with(".blocks.orderedList") =>
            {
                let numbered = t.ends_with(".blocks.orderedList");
                let items: Vec<Vec<Inline>> = block
                    .get("children")
                    .and_then(|c| c.as_array())
                    .map(|children| {
                        children
                            .iter()
                            .filter_map(|child| {
                                let content = child.get("content")?;
                                let item_text = content
                                    .get("plaintext")
                                    .and_then(|t| t.as_str())
                                    .unwrap_or("");
                                let inlines =
                                    apply_facets_to_inlines(item_text, content.get("facets"));
                                if inlines.is_empty() {
                                    None
                                } else {
                                    Some(inlines)
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                if items.is_empty() {
                    None
                } else if numbered {
                    Some(Block::OrderedList(items))
                } else {
                    Some(Block::UnorderedList(items))
                }
            }
            _ => {
                if !plaintext.is_empty() {
                    Some(Block::Paragraph(vec![Inline::Text(plaintext.to_string())]))
                } else {
                    None
                }
            }
        }
    }

    fn apply_facets_to_inlines(
        text: &str,
        facets: Option<&serde_json::Value>,
    ) -> Vec<Inline> {
        let facets = match facets.and_then(|f| f.as_array()) {
            Some(f) if !f.is_empty() => f,
            _ => return vec![Inline::Text(text.to_string())],
        };

        let mut sorted: Vec<&serde_json::Value> = facets.iter().collect();
        sorted.sort_by_key(|f| {
            f.get("index")
                .and_then(|i| i.get("byteStart"))
                .and_then(|b| b.as_u64())
                .unwrap_or(0)
        });

        let mut result = Vec::new();
        let mut last_end: usize = 0;
        let text_len = text.len();
        let text_bytes = text.as_bytes();

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
                let seg =
                    std::str::from_utf8(&text_bytes[last_end..start]).unwrap_or("");
                if !seg.is_empty() {
                    result.push(Inline::Text(seg.to_string()));
                }
            }

            let segment =
                std::str::from_utf8(&text_bytes[start..end]).unwrap_or("");
            if !segment.is_empty() {
                if let Some(url) = uri {
                    result.push(Inline::Link {
                        url: url.to_string(),
                        children: vec![Inline::Text(segment.to_string())],
                    });
                } else {
                    result.push(Inline::Text(segment.to_string()));
                }
            }

            last_end = end;
        }

        if last_end < text_len {
            let seg =
                std::str::from_utf8(&text_bytes[last_end..]).unwrap_or("");
            if !seg.is_empty() {
                result.push(Inline::Text(seg.to_string()));
            }
        }

        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use live::*;
