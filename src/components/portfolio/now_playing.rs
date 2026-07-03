use dioxus::prelude::*;

use ui::icons::MusicNoteIcon;
use crate::lastfm::{self, Track};

const BARS: usize = 30;
const WAVEFORM_STYLE: &str = r#"
@keyframes bar-wave {
  0%, 100% { transform: scaleY(0.4); }
  50% { transform: scaleY(1); }
}
@keyframes bar-idle {
  0%, 100% { transform: scaleY(0.3); }
  50% { transform: scaleY(0.5); }
}
.waveform-wrapper {
  display: flex;
  align-items: center;
  gap: 1px;
  height: 24px;
}
.waveform-bar {
  width: 2px;
  border-radius: 2px;
  transform-origin: bottom;
  height: 100%;
}
.waveform-bar.playing {
  animation: bar-wave 1.2s ease-in-out infinite;
}
.waveform-bar.idle {
  opacity: 0.35;
  animation: bar-idle 3s ease-in-out infinite;
  animation-delay: calc(var(--i) * 0.1s);
}
.waveform-bar.playing:nth-child(odd) {
  animation-duration: 1.1s;
}
.waveform-bar.playing:nth-child(3n) {
  animation-duration: 1.3s;
}
.waveform-bar.playing:nth-child(5n) {
  animation-duration: 1.0s;
}
.waveform-bar.playing:nth-child(7n) {
  animation-duration: 1.4s;
}
"#;

fn track_image(track: &Track) -> Option<&str> {
    track
        .image
        .last()
        .or_else(|| track.image.first())
        .map(|img| img.text.as_str())
}

fn track_artist(track: &Track) -> &str {
    &track.artist.text
}

fn track_name(track: &Track) -> &str {
    &track.name
}

#[component]
pub fn AlbumArt(
    src: String,
    class: String,
    fallback_class: String,
    alt: String,
    icon_class: String,
) -> Element {
    let mut failed = use_signal(|| src.is_empty());

    if failed() {
        rsx! {
            div { class: "{fallback_class}",
                MusicNoteIcon { class: "{icon_class}" }
            }
        }
    } else {
        rsx! {
            img {
                class: "{class}",
                src: "{src}",
                alt: "{alt}",
                onerror: move |_| *failed.write() = true,
            }
        }
    }
}

#[component]
pub fn NowPlaying() -> Element {
    let data = use_server_future(lastfm::fetch_lastfm_tracks);

    match data {
        Ok(resource) => match &*resource.read() {
            Some(Ok(d)) => render_now_playing(d),
            Some(Err(_)) => render_empty(),
            None => render_loading(),
        },
        Err(_) => render_empty(),
    }
}

fn render_loading() -> Element {
    rsx! {
        document::Style { {WAVEFORM_STYLE} }
        a {
            class: "group relative flex flex-col overflow-hidden rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
            href: "/music",
            div { class: "flex items-center gap-3",
                div { class: "bg-muted rounded-lg size-12 shrink-0 animate-pulse" }
                div { class: "flex-1 min-w-0 space-y-1.5",
                    div { class: "bg-muted rounded h-3 w-3/4 animate-pulse" }
                    div { class: "bg-muted rounded h-3 w-1/2 animate-pulse" }
                }
            }
        }
    }
}

fn render_empty() -> Element {
    rsx! {
        document::Style { {WAVEFORM_STYLE} }
        a {
            class: "group relative flex flex-col overflow-hidden rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
            href: "/music",
            div { class: "flex items-center gap-3",
                div { class: "flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted",
                    MusicNoteIcon { class: "size-4 text-muted-foreground" }
                }
                p { class: "font-mono text-xs text-muted-foreground",
                    "not listening right now"
                }
            }
        }
    }
}

fn render_now_playing(data: &lastfm::NowPlayingData) -> Element {
    let is_playing = data.now_playing.is_some();
    let track = match data
        .now_playing
        .as_ref()
        .or_else(|| data.recent_tracks.first())
    {
        Some(t) => t,
        None => return render_empty(),
    };
    let img = track_image(track).unwrap_or("").to_string();
    let bar_color = data.dominant_color.as_deref().unwrap_or("var(--neon-pink)");

    rsx! {
        document::Style { {WAVEFORM_STYLE} }
        a {
            class: "group relative flex flex-col overflow-hidden rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
            href: "/music",
            div { class: "flex items-center gap-3",
                div { class: "relative shrink-0",
                    AlbumArt {
                        src: img.clone(),
                        class: "size-12 rounded-lg border border-border object-cover".to_string(),
                        fallback_class: "flex size-12 items-center justify-center rounded-lg bg-muted".to_string(),
                        alt: format!("{} album art", track_name(track)),
                        icon_class: "size-4 text-muted-foreground".to_string(),
                    }
                }

                div { class: "flex-1 min-w-0",
                    div { class: "flex items-baseline gap-2",
                        if is_playing {
                            span { class: "font-mono text-[0.65rem] uppercase tracking-wider text-accent",
                                "now playing"
                            }
                        } else {
                            span { class: "font-mono text-[0.65rem] uppercase tracking-wider text-muted-foreground",
                                "last played"
                            }
                        }
                    }
                    p { class: "mt-0.5 truncate font-semibold text-sm text-foreground",
                        title: "{track_name(track)}",
                        "{track_name(track)}"
                    }
                    p { class: "truncate font-mono text-xs text-muted-foreground",
                        title: "{track_artist(track)}",
                        "{track_artist(track)}"
                    }
                }

                if is_playing {
                    div { class: "waveform-wrapper shrink-0 hidden md:flex",
                        for i in 0..BARS {
                            div {
                                class: "waveform-bar playing",
                                style: "--i: {i}; background-color: {bar_color}",
                            }
                        }
                    }
                } else {
                    div { class: "waveform-wrapper shrink-0 hidden md:flex opacity-35",
                        for i in 0..BARS {
                            div {
                                class: "waveform-bar idle",
                                style: "--i: {i}; background-color: {bar_color}",
                            }
                        }
                    }
                }
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
pub fn NowPlayingStats() -> Element {
    let data = use_server_future(lastfm::fetch_lastfm_tracks);

    match data {
        Ok(resource) => match &*resource.read() {
            Some(Ok(d)) => rsx! {
                a {
                    class: "group relative flex flex-col overflow-hidden rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/50",
                    href: "/music",
                    div { class: "flex flex-row items-center justify-around gap-6",
                        div { class: "flex flex-col",
                            dt { class: "font-mono text-[0.65rem] uppercase tracking-wider text-muted-foreground",
                                "scrobbles"
                            }
                            dd { class: "mt-0.5 font-display text-2xl font-bold text-foreground",
                                "{render_scrobbles(d.total_scrobbles)}"
                            }
                        }
                        div { class: "flex flex-col",
                            dt { class: "font-mono text-[0.65rem] uppercase tracking-wider text-muted-foreground",
                                "today"
                            }
                            dd { class: "mt-0.5 font-display text-2xl font-bold text-foreground",
                                "{d.scrobbles_today}"
                            }
                        }
                    }
                }
            },
            _ => rsx! {},
        },
        _ => rsx! {},
    }
}
