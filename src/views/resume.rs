use dioxus::prelude::*;

use crate::components::portfolio::{CHANNELS, Channel};
use crate::utils;

const RESUME_URL: &str =
    "https://gist.githubusercontent.com/metruzanca/751361e5ba58ad06f361ebd430ae6e10/raw/resume.json";

#[server]
pub async fn fetch_resume_server() -> Result<ResumeData, ServerFnError> {
    let body = ureq::get(RESUME_URL)
        .call()
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .into_string()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    serde_json::from_str(&body).map_err(|e| ServerFnError::new(e.to_string()))
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResumeData {
    pub basics: Basics,
    #[serde(default)]
    pub work: Vec<Work>,
    #[serde(default)]
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Basics {
    pub name: String,
    pub label: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub location: Option<Location>,
    #[serde(default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub network: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Work {
    pub name: String,
    pub position: String,
    #[serde(default)]
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(default)]
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Skill {
    pub name: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}

const PRINT_CSS: &str = r#"
  @page { margin: 0.5in; size: letter; }
  body {
    background: #fff !important;
    color: #111 !important;
    font-size: 10pt;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif !important;
    line-height: 1.35;
  }
  header.sticky, footer, nav, .synth-grid, .synth-scanlines { display: none !important; }
  *, *::before, *::after {
    text-shadow: none !important;
    box-shadow: none !important;
  }
  a { color: #111 !important; text-decoration: none; }

  .resume-article { max-width: none !important; padding: 0 !important; }

  .bg-card, .bg-secondary, .bg-background, [class*="bg-"] { background: transparent !important; }
  .border-border, [class*="border-"] { border-color: #333 !important; }
  .rounded-xl, [class*="rounded-"] { border-radius: 0 !important; }

  h1 { font-size: 16pt !important; margin: 0 0 2pt !important; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif !important; }
  h2 { font-size: 11pt !important; margin: 14pt 0 6pt !important; padding-bottom: 2pt; border-bottom: 0.5pt solid #333 !important; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif !important; }
  h3 { font-size: 10pt !important; margin: 0 !important; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif !important; }
  .font-heading, .font-display, .font-mono { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif !important; }

  h1, h2, h3, h4, h5, h6, p, span, li, a, div, time { color: #000 !important; }
  .text-foreground, .text-accent, .text-primary, .text-muted-foreground,
  .text-neon-cyan, .text-neon-pink, .text-neon-purple, .text-neon-amber,
  [class*="text-glow"] { color: #000 !important; text-shadow: none !important; }

  .mb-16 { margin-bottom: 8pt !important; }
  .mb-8 { margin-bottom: 0 !important; }
  .mb-6 { margin-bottom: 4pt !important; }
  .mb-3 { margin-bottom: 2pt !important; }
  .mt-6 { margin-top: 4pt !important; }
  .mt-3 { margin-top: 2pt !important; }
  .mt-4 { margin-top: 3pt !important; }
  .gap-10 { gap: 4pt !important; }
  .gap-1 { gap: 0 !important; }
  .gap-2 { gap: 4pt !important; }
  .space-y-1\\.5 > * + * { margin-top: 1pt !important; }
  .py-3\\.5 { padding-top: 3pt !important; padding-bottom: 3pt !important; }

  .border-l-2 { border-left-width: 0 !important; }
  .pl-6 { padding-left: 0 !important; }
  .size-2, .size-1, .size-1\\.5 { display: none !important; }
  .absolute { position: static !important; }
  .shrink-0 { flex-shrink: 1 !important; }

  .flex-col { flex-direction: row !important; flex-wrap: wrap !important; }
  .sm\\:flex-row { flex-direction: row !important; }
  .flex { display: flex !important; }
  .flex-wrap { flex-wrap: wrap !important; }

  ul { padding-left: 14pt !important; margin: 2pt 0 !important; }
  li { margin-bottom: 1pt !important; line-height: 1.3 !important; }
  li span:first-child { display: none !important; }

  .px-4 { padding-left: 0 !important; padding-right: 0 !important; }
  .px-6 { padding-left: 0 !important; padding-right: 0 !important; }
  .p-6 { padding: 4pt 0 !important; }
  .px-2\\.5 { padding-left: 0 !important; padding-right: 0 !important; }
  .py-1 { padding-top: 1pt !important; padding-bottom: 1pt !important; }

  .w-28 { width: auto !important; }
  .w-14 { width: auto !important; }

  .gap-x-6 { column-gap: 8pt !important; }
  .gap-y-1\\.5 { row-gap: 0 !important; }

  .text-sm { font-size: 10pt !important; }
  .text-xs { font-size: 9pt !important; }
  .text-lg { font-size: 10pt !important; }
  .text-xl { font-size: 11pt !important; }
  .text-2xl { font-size: 11pt !important; }
  .text-4xl { font-size: 16pt !important; }
  .text-5xl { font-size: 16pt !important; }
  .leading-relaxed { line-height: 1.3 !important; }
  .tracking-tight { letter-spacing: normal !important; }
  .tracking-widest { letter-spacing: 0.05em !important; }

  .uppercase { text-transform: uppercase; font-size: 8pt !important; letter-spacing: 0.08em !important; }

  .from-primary\\/5 { --tw-gradient-from: transparent !important; }
  .to-primary\\/0 { --tw-gradient-to: transparent !important; }

  .hidden.sm\\:block { display: inline !important; }

  .page-break { page-break-before: always; }
"#;

#[component]
pub fn ResumePage() -> Element {
    let resume = use_server_future(fetch_resume_server);

    match resume {
        Ok(resource) => match &*resource.read() {
            Some(Ok(data)) => {
                rsx! {
                    document::Style { media: "print", { PRINT_CSS } }
                    article { class: "mx-auto max-w-3xl px-4 pt-12 pb-24 resume-article",
                        ResumeHeader { basics: data.basics.clone() }
                        WorkSection { work: data.work.clone() }
                        SkillsSection { skills: data.skills.clone() }
                    }
                }
            }
            Some(Err(e)) => {
                rsx! {
                    div { class: "flex items-center justify-center py-24",
                        p { class: "font-mono text-sm text-muted-foreground",
                            "Failed to load resume: {e}"
                        }
                    }
                }
            }
            None => {
                rsx! {
                    div { class: "flex items-center justify-center py-24",
                        div { class: "font-mono text-sm text-muted-foreground animate-pulse",
                            "Loading resume\u{2026}"
                        }
                    }
                }
            }
        },
        Err(e) => {
            rsx! {
                div { class: "flex items-center justify-center py-24",
                    p { class: "font-mono text-sm text-muted-foreground",
                        "Failed to load resume: {e}"
                    }
                }
            }
        }
    }
}

#[component]
fn ResumeHeader(basics: Basics) -> Element {
    rsx! {
        header { class: "mb-16",
            div { class: "flex flex-col gap-1",
                h1 { class: "font-heading text-4xl font-extrabold tracking-tight text-glow-pink md:text-5xl",
                    "{basics.name}"
                }
                p { class: "font-heading text-xl font-semibold text-foreground md:text-2xl",
                    "{basics.label}"
                }
            }

            div { class: "mt-6 flex flex-wrap gap-x-6 gap-y-1.5",
                for ch in CHANNELS {
                    if ch.is_mail {
                        span { class: "font-mono text-sm text-muted-foreground",
                            "{ch.handle}"
                        }
                    } else if ch.label != "calendar" {
                        a {
                            class: "font-mono text-sm text-muted-foreground hover:text-accent transition-colors",
                            href: "{ch.href}",
                            target: "_blank",
                            rel: "noreferrer",
                            "{format_channel_label(ch)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WorkSection(work: Vec<Work>) -> Element {
    rsx! {
        section { class: "mb-16",
            h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-8",
                span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                "Experience"
            }

            div { class: "flex flex-col gap-10",
                for (_i, job) in work.iter().enumerate() {
                    div { class: "relative pl-6 border-l-2 border-border",
                        div { class: "absolute -left-[5px] top-1.5 size-2 rounded-full bg-accent" }

                        div { class: "flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-1",
                            div {
                                h3 { class: "font-heading text-lg font-semibold text-foreground",
                                    "{job.position}"
                                }
                                div { class: "flex items-center gap-2 flex-wrap",
                                    if !job.url.is_empty() {
                                        a {
                                            class: "font-mono text-sm text-accent hover:underline",
                                            href: "{job.url}",
                                            target: "_blank",
                                            rel: "noreferrer",
                                            "{job.name}"
                                        }
                                    } else {
                                        span { class: "font-mono text-sm text-accent",
                                            "{job.name}"
                                        }
                                    }
                                    if let Some(ref loc) = job.location {
                                        span { class: "font-mono text-xs text-muted-foreground",
                                            "\u{00B7} {loc}"
                                        }
                                    }
                                }
                            }
                            span { class: "font-mono text-xs text-muted-foreground shrink-0",
                                "{utils::datetime::format_date_range(&job.start_date, &job.end_date)}"
                            }
                        }

                        if !job.highlights.is_empty() {
                            ul { class: "mt-3 space-y-1.5",
                                for highlight in &job.highlights {
                                    li { class: "flex gap-2 text-sm text-muted-foreground leading-relaxed",
                                        span { class: "mt-1.5 size-1 shrink-0 rounded-full bg-border" }
                                        span { "{highlight}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillsSection(skills: Vec<Skill>) -> Element {
    if skills.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { class: "mb-16",
            h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                "Skills"
            }

            div { class: "rounded-xl border border-border bg-card p-6",
                for (idx, category) in skills.iter().enumerate() {
                    div { class: if idx > 0 { "mt-4" },
                        h3 { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground mb-3",
                            "{category.name}"
                        }
                        div { class: "flex flex-wrap gap-2",
                            for keyword in &category.keywords {
                                span { class: "inline-flex items-center rounded-md border border-border bg-secondary px-2.5 py-1 font-mono text-xs text-secondary-foreground hover:border-accent/40 transition-colors",
                                    "{keyword}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_channel_label(ch: &Channel) -> String {
    let url = ch.href.trim_start_matches("https://").trim_start_matches("http://");
    let url = url.trim_start_matches("www.");
    let (domain, path) = match url.find('/') {
        Some(idx) => (&url[..idx], &url[idx + 1..]),
        None => (url, ""),
    };
    let domain = match domain.rfind('.') {
        Some(idx) => &domain[..idx],
        None => domain,
    };
    let user = path.trim_end_matches('/').rsplit('/').next().unwrap_or("");
    if user.is_empty() {
        domain.to_lowercase()
    } else {
        format!("{}:{}", domain.to_lowercase(), user)
    }
}


