use dioxus::prelude::*;

use crate::components::portfolio::PortfolioNav;
use crate::Route;

#[component]
pub fn SiteLayout() -> Element {
    rsx! {
        div { class: "relative min-h-screen bg-background",
            div { class: "pointer-events-none fixed inset-0 synth-grid opacity-40 [mask-image:radial-gradient(ellipse_at_top,black,transparent_70%)]" }
            PortfolioNav {}
            Outlet::<Route> {}
        }
    }
}
