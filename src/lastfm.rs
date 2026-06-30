use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub name: String,
    pub artist: Artist,
    pub album: Album,
    pub url: String,
    pub image: Vec<Image>,
    #[serde(default)]
    pub date: Option<Date>,
    #[serde(rename = "@attr")]
    pub attr: Option<TrackAttr>,
    #[serde(default)]
    pub formatted_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Album {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    #[serde(rename = "#text")]
    pub text: String,
    pub size: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Date {
    #[serde(rename = "#text")]
    pub text: String,
    pub uts: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrackAttr {
    #[serde(default)]
    pub nowplaying: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct RecentTracksResponse {
    #[serde(rename = "recenttracks")]
    recent_tracks: RecentTracks,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct RecentTracks {
    #[serde(default)]
    track: Vec<Track>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct UserInfoResponse {
    user: UserInfo,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct UserInfo {
    #[serde(default)]
    playcount: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NowPlayingData {
    pub now_playing: Option<Track>,
    pub recent_tracks: Vec<Track>,
    pub dominant_color: Option<String>,
    pub total_scrobbles: u64,
    pub scrobbles_today: u64,
}

const LASTFM_BASE: &str = "https://ws.audioscrobbler.com/2.0/";

#[server]
pub async fn fetch_lastfm_tracks() -> Result<NowPlayingData, ServerFnError> {
    let api_key = std::env::var("LASTFM_API_KEY")
        .map_err(|_| ServerFnError::new("LASTFM_API_KEY not set"))?;
    let username = std::env::var("LASTFM_USERNAME")
        .map_err(|_| ServerFnError::new("LASTFM_USERNAME not set"))?;

    let now_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let today_start = now_ts - (now_ts % 86400);

    let recent_url = format!(
        "{base}?method=user.getrecenttracks&user={user}&api_key={key}&format=json&limit=200",
        base = LASTFM_BASE,
        user = username,
        key = api_key,
    );

    let user_url = format!(
        "{base}?method=user.getinfo&user={user}&api_key={key}&format=json",
        base = LASTFM_BASE,
        user = username,
        key = api_key,
    );

    let (recent_body, user_body) = match (
        ureq::get(&recent_url).call(),
        ureq::get(&user_url).call(),
    ) {
        (Ok(r), Ok(u)) => (
            r.into_string()
                .map_err(|e| ServerFnError::new(format!("Failed to read response: {e}")))?,
            u.into_string()
                .map_err(|e| ServerFnError::new(format!("Failed to read response: {e}")))?,
        ),
        _ => return Err(ServerFnError::new("Last.fm request failed")),
    };

    let recent_response: RecentTracksResponse =
        serde_json::from_str(&recent_body).map_err(|e| ServerFnError::new(format!("Failed to parse: {e}")))?;

    let user_response: UserInfoResponse =
        serde_json::from_str(&user_body).unwrap_or(UserInfoResponse {
            user: UserInfo {
                playcount: "0".to_string(),
            },
        });

    let total_scrobbles: u64 = user_response.user.playcount.parse().unwrap_or(0);

    let all_tracks = recent_response.recent_tracks.track;

    let scrobbles_today = all_tracks
        .iter()
        .filter(|t| {
            t.date.as_ref().map_or(false, |d| {
                d.uts.parse::<i64>().unwrap_or(0) >= today_start
            })
        })
        .count() as u64;

    let now_playing = all_tracks
        .first()
        .filter(|t| t.attr.as_ref().map_or(false, |a| a.nowplaying == "true"))
        .cloned();

    let recent_start = if now_playing.is_some() { 1 } else { 0 };
    let recent_tracks: Vec<Track> = all_tracks
        .into_iter()
        .skip(recent_start)
        .take(10)
        .map(|mut t| {
            t.formatted_time = t
                .date
                .as_ref()
                .and_then(|d| d.uts.parse::<i64>().ok())
                .map(|uts| relative_time(uts, now_ts));
            t
        })
        .collect();

    let dominant_color = color_track(&now_playing, &recent_tracks);

    Ok(NowPlayingData {
        now_playing,
        recent_tracks,
        dominant_color,
        total_scrobbles,
        scrobbles_today,
    })
}

#[cfg(feature = "server")]
fn track_largest_image(track: &Track) -> Option<&str> {
    track.image.last().or_else(|| track.image.first()).map(|img| img.text.as_str())
}

#[cfg(feature = "server")]
fn color_track(now_playing: &Option<Track>, recent: &[Track]) -> Option<String> {
    let track = now_playing.as_ref().or_else(|| recent.first())?;
    let img_url = track_largest_image(track)?;
    if img_url.is_empty() {
        return None;
    }
    extract_dominant_color(img_url)
}

#[cfg(feature = "server")]
fn relative_time(uts: i64, now: i64) -> String {
    let diff = now - uts;
    match diff {
        d if d < 60 => "just now".into(),
        d if d < 3600 => format!("{}m ago", d / 60),
        d if d < 7200 => "1h ago".into(),
        d if d < 86400 => format!("{}h ago", d / 3600),
        d if d < 172800 => "yesterday".into(),
        d if d < 604800 => format!("{}d ago", d / 86400),
        d if d < 2592000 => format!("{}w ago", d / 604800),
        _ => {
            let months = diff / 2592000;
            format!("{}mo ago", months)
        }
    }
}

#[cfg(feature = "server")]
fn extract_dominant_color(image_url: &str) -> Option<String> {
    use std::io::Read;

    let response = ureq::get(image_url).call().ok()?;
    let mut buf = Vec::new();
    response.into_reader().read_to_end(&mut buf).ok()?;
    let img = image::load_from_memory(&buf).ok()?;
    let small = img.resize_exact(10, 10, image::imageops::FilterType::Nearest);
    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);
    let mut count = 0u64;
    for pixel in small.to_rgb8().pixels() {
        r += pixel[0] as u64;
        g += pixel[1] as u64;
        b += pixel[2] as u64;
        count += 1;
    }
    if count > 0 {
        Some(format!("#{:02x}{:02x}{:02x}", r / count, g / count, b / count))
    } else {
        None
    }
}
