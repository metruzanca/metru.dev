use dioxus::prelude::*;

use components::layout::SiteLayout;
use views::{About, BlogList, BlogPost, DesignSystem, HowIWork, Landing, Music, ProjectsList, ResumePage};

mod blog;
mod components;
mod github;
mod lastfm;
mod utils;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(SiteLayout)]
        #[route("/")]
        Landing {},
        #[redirect("/index.html", || Route::Landing {})]
        #[route("/projects")]
        ProjectsList {},
        #[route("/blog")]
        BlogList {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
        #[route("/music")]
        Music {},
        #[route("/how-i-work")]
        HowIWork {},
        #[route("/about")]
        About {},
    #[route("/design-system")]
    DesignSystem {},
    #[route("/resume")]
    ResumePage {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.push(Route::Landing {});
    });
    rsx! {}
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Geist:wght@100..900&family=Geist+Mono:wght@100..900&family=Orbitron:wght@500;600;700;800;900&display=swap",
        }

        document::Script {
            r#type: "text/javascript",
            "document.documentElement.lang = 'en';",
        }
        document::Meta { name: "description", content: "Sam Zanca — Full-stack engineer building fast, reliable products with Rust and modern web technologies. Portfolio, blog, and projects." }
        document::Meta { name: "color-scheme", content: "dark" }
        document::Meta { name: "theme-color", content: "#0c0a1f" }

        // Open Graph
        document::Meta { property: "og:title", content: "Sam Zanca — Full-Stack Engineer" }
        document::Meta { property: "og:description", content: "Building fast, reliable products with Rust and modern web technologies." }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "https://metru.dev" }
        document::Meta { property: "og:image", content: "https://metru.dev/assets/synthwave-horizon.png" }

        // Twitter Card
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "Sam Zanca — Full-Stack Engineer" }
        document::Meta { name: "twitter:description", content: "Building fast, reliable products with Rust and modern web technologies." }
        document::Meta { name: "twitter:image", content: "https://metru.dev/assets/synthwave-horizon.png" }

        // Umami – Privacy-Focused Web Analytics
        document::Script {
            defer: true,
            src: "https://umami-production-6edd.up.railway.app/script.js",
            r#type: "text/javascript",
            "data-website-id": "b3d56abc-c1d1-4921-8372-36ab2e5abc7d",
        }

        Router::<Route> {}
    }
}
