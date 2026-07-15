use std::sync::OnceLock;

pub fn site_domain() -> &'static str {
    static DOMAIN: OnceLock<String> = OnceLock::new();
    DOMAIN.get_or_init(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var("SITE_DOMAIN")
                .ok()
                .filter(|s| !s.is_empty())
                .or_else(|| std::env::var("RAILWAY_PUBLIC_DOMAIN").ok().filter(|s| !s.is_empty()))
                .unwrap_or_else(|| "metru.dev".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            "metru.dev".to_string()
        }
    })
}

pub fn atproto_handle() -> &'static str {
    static HANDLE: OnceLock<String> = OnceLock::new();
    HANDLE.get_or_init(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var("ATPROTO_HANDLE")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "metru.dev".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            "metru.dev".to_string()
        }
    })
}
