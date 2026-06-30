use dioxus::prelude::*;

use crate::components::portfolio::AlbumArt;
use crate::lastfm::{self, Track};

const WAVEFORM_STYLE: &str = r#"
@keyframes bar-wave {
  0%, 100% { transform: scaleY(0.4); }
  50% { transform: scaleY(1); }
}
@keyframes bar-idle {
  0%, 100% { transform: scaleY(0.3); }
  50% { transform: scaleY(0.5); }
}
.music-waveform {
  display: flex;
  align-items: center;
  gap: 2px;
  height: 40px;
}
.music-bar {
  width: 3px;
  border-radius: 2px;
  transform-origin: bottom;
  height: 100%;
}
.music-bar.playing {
  animation: bar-wave 1.2s ease-in-out infinite;
}
.music-bar.idle {
  opacity: 0.35;
}
.music-bar.playing:nth-child(odd) { animation-duration: 1.1s; }
.music-bar.playing:nth-child(3n) { animation-duration: 1.3s; }
.music-bar.playing:nth-child(5n) { animation-duration: 1.0s; }
.music-bar.playing:nth-child(7n) { animation-duration: 1.4s; }
"#;

#[component]
pub fn Music() -> Element {
    let data = use_server_future(lastfm::fetch_lastfm_tracks);

    rsx! {
        document::Style { {WAVEFORM_STYLE} }

        main { class: "mx-auto max-w-3xl px-4 pt-14 pb-24 md:pt-20",
            header { class: "mb-12",
                h1 { class: "font-display text-4xl font-extrabold uppercase text-foreground md:text-6xl",
                    span { class: "text-primary text-glow-pink", "music " }
                    "I like"
                }
                p { class: "mt-4 max-w-xl font-mono text-sm text-muted-foreground",
                    "What I'm listening to, powered by "
                    a {
                        class: "text-accent hover:underline",
                        href: "https://last.fm",
                        target: "_blank",
                        rel: "noreferrer",
                        "Last.fm"
                    }
                }
            }

            match data {
                Ok(resource) => match &*resource.read() {
                    Some(Ok(d)) => rsx! { MusicContent { data: d.clone() } },
                    Some(Err(_)) => rsx! {
                        div { class: "flex items-center justify-center py-24",
                            p { class: "font-mono text-sm text-muted-foreground",
                                "Couldn't fetch listening data. Make sure LASTFM_API_KEY and LASTFM_USERNAME are set."
                            }
                        }
                    },
                    None => rsx! {
                        div { class: "flex items-center justify-center py-24",
                            div { class: "font-mono text-sm text-muted-foreground animate-pulse",
                                "Loading listening data\u{2026}"
                            }
                        }
                    },
                },
                Err(_) => rsx! {
                    div { class: "flex items-center justify-center py-24",
                        p { class: "font-mono text-sm text-muted-foreground",
                            "Couldn't fetch listening data."
                        }
                    }
                },
            }
        }
    }
}

fn render_scrobbles(count: u64) -> String {
    if count >= 1000 {
        format!("{:.1}k", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}

#[component]
fn MusicContent(data: lastfm::NowPlayingData) -> Element {
    let is_playing = data.now_playing.is_some();
    let bar_color = data.dominant_color.as_deref().unwrap_or("var(--neon-pink)");

    rsx! {
        // Stats
        section { class: "mb-12",
            div { class: "grid grid-cols-2 gap-4",
                div { class: "rounded-xl border border-border bg-card p-5",
                    dt { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                        "total scrobbles"
                    }
                    dd { class: "mt-1 font-display text-3xl font-bold text-foreground",
                        "{render_scrobbles(data.total_scrobbles)}"
                    }
                }
                div { class: "rounded-xl border border-border bg-card p-5",
                    dt { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                        "today"
                    }
                    dd { class: "mt-1 font-display text-3xl font-bold text-foreground",
                        "{data.scrobbles_today}"
                    }
                }
            }
        }

        section { class: "mb-12",
            h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                "Now"
            }

            div { class: "rounded-xl border border-border bg-card p-6 md:p-8",
                div { class: "flex flex-col gap-6 md:flex-row md:items-center md:gap-8",
                    div { class: "relative shrink-0",
                        if let Some(ref track) = data.now_playing {
                            AlbumArt {
                                src: track_image(track).to_string(),
                                class: "size-44 md:size-52 rounded-xl border border-border object-cover".to_string(),
                                fallback_class: "flex size-44 md:size-52 items-center justify-center rounded-xl bg-muted".to_string(),
                                alt: format!("{} album art", track.name),
                                icon_class: "size-10 text-muted-foreground".to_string(),
                            }
                        } else if let Some(ref track) = data.recent_tracks.first() {
                            AlbumArt {
                                src: track_image(track).to_string(),
                                class: "size-44 md:size-52 rounded-xl border border-border object-cover".to_string(),
                                fallback_class: "flex size-44 md:size-52 items-center justify-center rounded-xl bg-muted".to_string(),
                                alt: format!("{} album art", track.name),
                                icon_class: "size-10 text-muted-foreground".to_string(),
                            }
                        }
                    }

                    div { class: "flex-1 min-w-0",
                        if let Some(ref track) = data.now_playing {
                            div { class: "flex items-center gap-2 mb-1",
                                span { class: "font-mono text-[0.7rem] uppercase tracking-widest text-accent",
                                    "now playing"
                                }
                                span { class: "relative flex size-1.5",
                                    span { class: "absolute inline-flex size-full animate-ping rounded-full bg-accent opacity-60" }
                                    span { class: "relative size-1.5 rounded-full bg-accent" }
                                }
                            }

                            h3 { class: "text-2xl font-bold text-foreground truncate",
                                "{track.name}"
                            }
                            p { class: "mt-1 text-lg text-muted-foreground",
                                "{track.artist.text}"
                            }
                            if !track.album.text.is_empty() {
                                p { class: "mt-1 font-mono text-sm text-muted-foreground",
                                    "{track.album.text}"
                                }
                            }
                        } else if let Some(ref track) = data.recent_tracks.first() {
                            h3 { class: "text-2xl font-bold text-foreground truncate",
                                "{track.name}"
                            }
                            p { class: "mt-1 text-lg text-muted-foreground",
                                "{track.artist.text}"
                            }
                            if !track.album.text.is_empty() {
                                p { class: "mt-1 font-mono text-sm text-muted-foreground",
                                    "{track.album.text}"
                                }
                            }
                        }
                    }
                }

                if is_playing {
                    div { class: "mt-6",
                        div { class: "music-waveform",
                            for i in 0..80 {
                                div {
                                    class: "music-bar playing",
                                    style: "--i: {i}; background-color: {bar_color}",
                                }
                            }
                        }
                    }
                }

                if is_playing {
                    p { class: "mt-3 font-mono text-[0.65rem] text-muted-foreground text-right",
                        "live"
                    }
                }
            }
        }

        // Recent tracks
        if !data.recent_tracks.is_empty() {
            section {
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                    "Recently played"
                }

                div { class: "rounded-xl border border-border bg-card divide-y divide-border overflow-hidden",
                    for track in &data.recent_tracks {
                        div { class: "flex items-center gap-4 px-5 py-3 hover:bg-muted/40 transition-colors",
                            AlbumArt {
                                src: track_image(track).to_string(),
                                class: "size-10 shrink-0 rounded border border-border object-cover".to_string(),
                                fallback_class: "flex size-10 shrink-0 items-center justify-center rounded bg-muted".to_string(),
                                alt: format!("{} album art", track.name),
                                icon_class: "size-4 text-muted-foreground".to_string(),
                            }

                            div { class: "flex-1 min-w-0",
                                p { class: "text-sm font-medium text-foreground truncate",
                                    "{track.name}"
                                }
                                p { class: "text-xs text-muted-foreground truncate font-mono",
                                    "{track.artist.text}"
                                }
                            }

                            if let Some(ref time) = track.formatted_time {
                                span { class: "shrink-0 font-mono text-xs text-muted-foreground",
                                    "{time}"
                                }
                            }

                            a {
                                class: "shrink-0 text-xs text-muted-foreground hover:text-accent transition-colors",
                                href: "{track.url}",
                                target: "_blank",
                                rel: "noreferrer",
                                "\u{2197}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn track_image(track: &Track) -> &str {
    track
        .image
        .iter()
        .rev()
        .find(|img| !img.text.is_empty())
        .or_else(|| track.image.first())
        .map(|img| img.text.as_str())
        .unwrap_or("")
}
