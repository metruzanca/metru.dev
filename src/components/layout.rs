use dioxus::prelude::*;

use crate::components::portfolio::PortfolioNav;
use crate::components::command_palette::CommandPalette;
use crate::Route;

#[component]
pub fn SiteLayout() -> Element {
    let palette_open = use_signal(|| false);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let mut p = palette_open;

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                move |e: web_sys::KeyboardEvent| {
                    if (e.ctrl_key() || e.meta_key()) && e.key() == "k" {
                        e.prevent_default();
                        let was_open = p();
                        p.set(!was_open);
                    }
                },
            )
                as Box<dyn FnMut(web_sys::KeyboardEvent)>);

            let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    rsx! {
        div { class: "relative min-h-screen bg-background",
            div { class: "pointer-events-none fixed inset-0 synth-grid opacity-40 [mask-image:radial-gradient(ellipse_at_top,black,transparent_70%)]" }
            PortfolioNav {}
            Outlet::<Route> {}
            CommandPalette { open: palette_open }
        }
    }
}
