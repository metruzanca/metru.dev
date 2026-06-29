use dioxus::prelude::*;

use components::layout::SiteLayout;
use views::{BlogList, BlogPost, DesignSystem, Landing, ProjectsList};

mod blog;
mod components;
mod github;
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
    #[route("/design-system")]
    DesignSystem {},
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

        document::Meta { name: "color-scheme", content: "dark" }
        document::Meta { name: "theme-color", content: "#0c0a1f" }

        Router::<Route> {}
    }
}
