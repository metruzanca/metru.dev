# Creating a Lab

Each lab is a standalone Dioxus component in its own workspace crate under `labs/<name>/`.

## Steps

### 1. Create the crate

```
labs/<name>/
├── Cargo.toml
└── src/
    └── lib.rs
```

```toml
[package]
name = "lab-<name>"       # e.g. lab-word-count, lab-timezones
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = { version = "0.7.1" }
ui = { path = "../../ui" }
# add extra deps as needed

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Location", "History"] }
js-sys = "0.3"
```

### 2. Write `lib.rs`

```rust
use dioxus::prelude::*;
use ui::LabMeta;

pub const META: LabMeta = LabMeta {
    slug: "<slug>",         // short slug, used in URL  e.g. "word-count", "tz"
    name: "<Name>",         // display name
    description: "<...>",   // one-line description
    tags: &["tool"],        // category tags
};

#[component]
pub fn App() -> Element {
    rsx! {
        div { class: "space-y-4",
            // your lab UI here
        }
    }
}
```

### 3. Register in the root app

**`Cargo.toml` (root)** — add dependency:
```toml
lab-<name> = { path = "./labs/<name>" }
```

**`src/labs.rs`** — add to `LABS` array:
```rust
LabInfo {
    meta: &lab_<name>::META,
    render: lab_<name>::App,
},
```

The route `/labs/:slug` and the `/labs` list page work automatically.

## Constraints & Patterns

### LabMeta fields
- `slug` — used in URL path, must be unique
- `name` — displayed on lab card and page heading
- `description` — one short sentence
- `tags` — static slice of category strings

### Dioxus 0.7 rules
- No `cx`, `Scope`, `use_state`. Use `use_signal`, `use_memo`, `use_effect`
- Props must be owned (`String` not `&str`). They implement `PartialEq + Clone`
- No `let` statements inside `for` loops within `rsx!` — precompute data before the block
- `EventHandlers` are passed as props; closures capture by `move`

### Browser-specific APIs
Use `#[cfg(target_arch = "wasm32")]` modules to gate `web-sys` / `js-sys` code:

```rust
#[cfg(not(target_arch = "wasm32"))]
mod browser {
    pub fn some_fn() {}
}

#[cfg(target_arch = "wasm32")]
mod browser {
    pub fn some_fn() { web_sys::window()... }
}
```

### URL state
Labs manage their own query params via `web-sys::window().location()` and `history.replace_state_with_url`. Read params in a `use_effect` on mount, write them back on state changes (also via `use_effect`).

### Hydration safety
Browser-only code (`window`, `navigator`, etc.) must run inside `use_effect`, not during the initial render. The server and first client render must produce identical HTML.

### Reference labs
- `labs/word-count/` — simplest lab (single component, no extra deps)
- `labs/timezones/` — full-featured lab with URL state, browser APIs, searchable dropdown, conditional compilation
