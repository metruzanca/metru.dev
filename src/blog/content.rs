use serde::{Deserialize, Serialize};

// ── Block-level content types ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Block {
    Heading {
        level: u8,
        children: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    CodeBlock {
        language: String,
        code: String,
    },
    UnorderedList(Vec<Vec<Inline>>),
    OrderedList(Vec<Vec<Inline>>),
    Blockquote(Vec<Block>),
    ThematicBreak,
    Raw(String),
}

// ── Inline content types ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Inline {
    Text(String),
    Strong(Vec<Inline>),
    Emphasis(Vec<Inline>),
    InlineCode(String),
    Link {
        url: String,
        children: Vec<Inline>,
    },
    Image {
        alt: String,
        src: String,
    },
    LineBreak,
}

// ── mdast conversion (markdown source files) ───────────────────────

pub fn from_mdast(root: &markdown::mdast::Node) -> Vec<Block> {
    match root {
        markdown::mdast::Node::Root(r) => r.children.iter().filter_map(block_from_mdast).collect(),
        other => block_from_mdast(other).into_iter().collect(),
    }
}

fn blocks_from_mdast_nodes(nodes: &[markdown::mdast::Node]) -> Vec<Block> {
    nodes.iter().filter_map(block_from_mdast).collect()
}

fn block_from_mdast(node: &markdown::mdast::Node) -> Option<Block> {
    match node {
        markdown::mdast::Node::Heading(h) => Some(Block::Heading {
            level: h.depth,
            children: inlines_from_nodes(&h.children),
        }),
        markdown::mdast::Node::Paragraph(p) => {
            Some(Block::Paragraph(inlines_from_nodes(&p.children)))
        }
        markdown::mdast::Node::Code(c) => Some(Block::CodeBlock {
            language: c.lang.clone().unwrap_or_default(),
            code: c.value.clone(),
        }),
        markdown::mdast::Node::ThematicBreak(_) => Some(Block::ThematicBreak),
        markdown::mdast::Node::Blockquote(bq) => {
            let children = blocks_from_mdast_nodes(&bq.children);
            if children.is_empty() {
                None
            } else {
                Some(Block::Blockquote(children))
            }
        }
        markdown::mdast::Node::List(list) => {
            let items: Vec<Vec<Inline>> = list
                .children
                .iter()
                .filter_map(|child| match child {
                    markdown::mdast::Node::ListItem(li) => {
                        let inlines = inlines_from_nodes(&li.children);
                        if inlines.is_empty() {
                            None
                        } else {
                            Some(inlines)
                        }
                    }
                    _ => None,
                })
                .collect();

            if items.is_empty() {
                None
            } else if list.ordered {
                Some(Block::OrderedList(items))
            } else {
                Some(Block::UnorderedList(items))
            }
        }
        markdown::mdast::Node::Html(h) => Some(Block::Raw(h.value.clone())),
        _ => None,
    }
}

fn inlines_from_nodes(nodes: &[markdown::mdast::Node]) -> Vec<Inline> {
    nodes.iter().filter_map(inline_from_mdast).collect()
}

fn inline_from_mdast(node: &markdown::mdast::Node) -> Option<Inline> {
    match node {
        markdown::mdast::Node::Text(t) => Some(Inline::Text(t.value.clone())),
        markdown::mdast::Node::Strong(s) => Some(Inline::Strong(inlines_from_nodes(&s.children))),
        markdown::mdast::Node::Emphasis(e) => {
            Some(Inline::Emphasis(inlines_from_nodes(&e.children)))
        }
        markdown::mdast::Node::InlineCode(c) => Some(Inline::InlineCode(c.value.clone())),
        markdown::mdast::Node::Link(l) => Some(Inline::Link {
            url: l.url.clone(),
            children: inlines_from_nodes(&l.children),
        }),
        markdown::mdast::Node::Image(img) => Some(Inline::Image {
            alt: img.alt.clone(),
            src: img.url.clone(),
        }),
        markdown::mdast::Node::Break(_) => Some(Inline::LineBreak),
        markdown::mdast::Node::Delete(d) => {
            Some(Inline::Strong(inlines_from_nodes(&d.children)))
        }
        _ => None,
    }
}

// ── HTML rendering ─────────────────────────────────────────────────

use std::sync::LazyLock;
use syntect::html::highlighted_html_for_string;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_newlines);
static THEME: LazyLock<syntect::highlighting::Theme> = LazyLock::new(|| {
    let ts = ThemeSet::load_defaults();
    ts.themes["base16-ocean.dark"].clone()
});

pub fn render_blocks(blocks: &[Block], asset_prefix: &str) -> String {
    let mut html = String::new();
    for block in blocks {
        match block {
            Block::Heading { level, children } => {
                let inner = render_inlines(children, asset_prefix);
                html.push_str(&format!("<h{level}>{inner}</h{level}>\n"));
            }
            Block::Paragraph(children) => {
                let inner = render_inlines(children, asset_prefix);
                html.push_str(&format!("<p>{inner}</p>\n"));
            }
            Block::CodeBlock { language, code } => {
                html.push_str(&render_code_block(language, code));
                html.push('\n');
            }
            Block::UnorderedList(items) => {
                html.push_str("<ul>\n");
                for item in items {
                    html.push_str("<li>");
                    html.push_str(&render_inlines(item, asset_prefix));
                    html.push_str("</li>\n");
                }
                html.push_str("</ul>\n");
            }
            Block::OrderedList(items) => {
                html.push_str("<ol>\n");
                for item in items {
                    html.push_str("<li>");
                    html.push_str(&render_inlines(item, asset_prefix));
                    html.push_str("</li>\n");
                }
                html.push_str("</ol>\n");
            }
            Block::Blockquote(children) => {
                let inner = render_blocks(children, asset_prefix);
                html.push_str(&format!("<blockquote>\n{inner}</blockquote>\n"));
            }
            Block::ThematicBreak => {
                html.push_str("<hr>\n");
            }
            Block::Raw(raw) => {
                html.push_str(raw);
                html.push('\n');
            }
        }
    }
    html
}

fn render_inlines(inlines: &[Inline], asset_prefix: &str) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) => out.push_str(&html_escape(t)),
            Inline::Strong(children) => {
                out.push_str("<strong>");
                out.push_str(&render_inlines(children, asset_prefix));
                out.push_str("</strong>");
            }
            Inline::Emphasis(children) => {
                out.push_str("<em>");
                out.push_str(&render_inlines(children, asset_prefix));
                out.push_str("</em>");
            }
            Inline::InlineCode(c) => {
                out.push_str("<code>");
                out.push_str(&html_escape(c));
                out.push_str("</code>");
            }
            Inline::Link { url, children } => {
                let inner = render_inlines(children, asset_prefix);
                out.push_str(&format!(
                    "<a href=\"{}\">{inner}</a>",
                    html_escape(url)
                ));
            }
            Inline::Image { alt, src } => {
                let rewritten = rewrite_image_src(src, asset_prefix);
                out.push_str(&format!(
                    "<img src=\"{}\" alt=\"{}\" />",
                    html_escape(&rewritten),
                    html_escape(alt)
                ));
            }
            Inline::LineBreak => {
                out.push_str("<br>\n");
            }
        }
    }
    out
}

fn render_code_block(language: &str, code: &str) -> String {
    let syntax = SYNTAXES
        .find_syntax_by_token(language)
        .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

    match highlighted_html_for_string(code, &SYNTAXES, syntax, &THEME) {
        Ok(highlighted) => {
            let lang_label = if language.is_empty() {
                String::new()
            } else {
                format!(
                    "<div class=\"code-lang\">{}</div>",
                    html_escape(language)
                )
            };
            format!("<div class=\"code-block-wrapper\">{lang_label}{highlighted}</div>")
        }
        Err(_) => {
            format!(
                "<pre><code class=\"language-{}\">{}</code></pre>",
                html_escape(language),
                html_escape(code)
            )
        }
    }
}

fn rewrite_image_src(src: &str, asset_prefix: &str) -> String {
    if let Some(rest) = src.strip_prefix("./_assets/") {
        format!("/assets/{asset_prefix}/{rest}")
    } else {
        src.to_string()
    }
}

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

// ── Plain text extraction (search, read time) ──────────────────────

pub fn blocks_to_plain_text(blocks: &[Block]) -> String {
    let mut text = String::new();
    for block in blocks {
        match block {
            Block::Heading { children, .. }
            | Block::Paragraph(children) => {
                text.push_str(&inlines_to_plain_text(children));
                text.push(' ');
            }
            Block::Blockquote(children) => {
                text.push_str(&blocks_to_plain_text(children));
                text.push(' ');
            }
            Block::CodeBlock { code, .. } => {
                text.push_str(code);
                text.push(' ');
            }
            Block::UnorderedList(items) | Block::OrderedList(items) => {
                for item in items {
                    text.push_str(&inlines_to_plain_text(item));
                    text.push(' ');
                }
            }
            Block::ThematicBreak | Block::Raw(_) => {}
        }
    }
    text
}

fn inlines_to_plain_text(inlines: &[Inline]) -> String {
    let mut text = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) | Inline::InlineCode(t) => text.push_str(t),
            Inline::Strong(children) | Inline::Emphasis(children) | Inline::Link { children, .. } => {
                text.push_str(&inlines_to_plain_text(children));
            }
            Inline::Image { alt, .. } => text.push_str(alt),
            Inline::LineBreak => text.push(' '),
        }
    }
    text
}

#[allow(dead_code)]
pub fn word_count(blocks: &[Block]) -> usize {
    let text = blocks_to_plain_text(blocks);
    text.split_whitespace().count()
}
