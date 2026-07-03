use dioxus::prelude::*;

#[component]
pub fn TriangleIcon(class: Option<String>, filled: Option<bool>) -> Element {
    let fill_val = if filled.unwrap_or(false) { "currentColor" } else { "none" };
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: fill_val,
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
        }
    }
}

#[component]
pub fn SearchIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "11", cy: "11", r: "8" }
            path { d: "m21 21-4.3-4.3" }
        }
    }
}

#[component]
pub fn BookOpenIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 7v14" }
            path { d: "M16 12h2" }
            path { d: "M16 8h2" }
            path { d: "M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z" }
        }
    }
}

#[component]
pub fn SparklesIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z" }
        }
    }
}

#[component]
pub fn PaletteIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "13.5", cy: "6.5", r: "1.5" }
            circle { cx: "17.5", cy: "10.5", r: "1.5" }
            circle { cx: "8.5", cy: "7.5", r: "1.5" }
            circle { cx: "6.5", cy: "12.5", r: "1.5" }
            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
        }
    }
}

#[component]
pub fn TypeIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "4 7 4 4 20 4 20 7" }
            line { x1: "9", x2: "15", y1: "20", y2: "20" }
            line { x1: "12", x2: "12", y1: "4", y2: "20" }
        }
    }
}

#[component]
pub fn ComponentIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M15.536 11.293a1 1 0 0 0 0 1.414l2.376 2.377a1 1 0 0 0 1.414 0l2.377-2.377a1 1 0 0 0 0-1.414l-2.377-2.377a1 1 0 0 0-1.414 0z" }
            path { d: "M2.297 11.293a1 1 0 0 0 0 1.414l2.377 2.377a1 1 0 0 0 1.414 0l2.377-2.377a1 1 0 0 0 0-1.414L6.088 8.916a1 1 0 0 0-1.414 0z" }
            path { d: "M8.916 17.912a1 1 0 0 0 0 1.415l2.377 2.376a1 1 0 0 0 1.414 0l2.377-2.376a1 1 0 0 0 0-1.415l-2.377-2.376a1 1 0 0 0-1.414 0z" }
            path { d: "M8.916 4.674a1 1 0 0 0 0 1.414l2.377 2.376a1 1 0 0 0 1.414 0l2.377-2.376a1 1 0 0 0 0-1.414l-2.377-2.377a1 1 0 0 0-1.414 0z" }
        }
    }
}

#[component]
pub fn PlusIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M5 12h14" }
            path { d: "M12 5v14" }
        }
    }
}

#[component]
pub fn PlayIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polygon { points: "6 3 20 12 6 21 6 3" }
        }
    }
}

#[component]
pub fn HeartIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
        }
    }
}

#[component]
pub fn ZapIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" }
        }
    }
}

#[component]
pub fn ArrowRightIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M5 12h14" }
            path { d: "m12 5 7 7-7 7" }
        }
    }
}

#[component]
pub fn CommandIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" }
        }
    }
}

#[component]
pub fn CircleIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
        }
    }
}

#[component]
pub fn MapPinIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" }
            circle { cx: "12", cy: "10", r: "3" }
        }
    }
}

#[component]
pub fn GitBranchIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "6", x2: "6", y1: "3", y2: "15" }
            circle { cx: "6", cy: "18", r: "3" }
            circle { cx: "18", cy: "6", r: "3" }
            path { d: "M18 9a9 9 0 0 1-9 9" }
        }
    }
}

#[component]
pub fn AtSignIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8" }
        }
    }
}

#[component]
pub fn SendIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M14.536 21.686a.5.5 0 0 0 .937-.024l6.5-19a.496.496 0 0 0-.635-.635l-19 6.5a.5.5 0 0 0-.024.937l7.93 3.18a2 2 0 0 1 1.112 1.113z" }
            path { d: "m21.854 2.147-10.94 10.939" }
        }
    }
}

#[component]
pub fn ArrowUpRightIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M7 17 17 7" }
            path { d: "M7 7h10v10" }
        }
    }
}

#[component]
pub fn StarIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
        }
    }
}

#[component]
pub fn MailIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "20", height: "16", x: "2", y: "4", rx: "2" }
            path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
        }
    }
}

#[component]
pub fn Gamepad2Icon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "6", x2: "10", y1: "11", y2: "11" }
            line { x1: "8", x2: "8", y1: "9", y2: "13" }
            line { x1: "15", x2: "15.01", y1: "12", y2: "12" }
            line { x1: "18", x2: "18.01", y1: "10", y2: "10" }
            path { d: "M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z" }
        }
    }
}

#[component]
pub fn XIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" }
        }
    }
}

#[component]
pub fn BlueSkyIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M12 10.8c-1.087-2.114-4.046-6.053-7.798-7.995C2.566.944 1.561 1.266.902 1.565.139 1.908 0 3.08 0 3.768c0 .69.378 5.65.624 6.479.815 2.736 3.713 3.66 6.383 3.364.136-.02.275-.039.415-.056-.138.022-.276.04-.415.056-3.912.58-7.387 2.005-2.83 7.078 5.013 5.19 6.87-1.113 7.823-4.308.953 3.195 2.05 9.271 7.733 4.308 4.267-4.308 1.172-6.498-2.74-7.078a8.741 8.741 0 0 1-.415-.056c.14.017.279.036.415.056 2.67.297 5.568-.628 6.383-3.364.246-.828.624-5.79.624-6.478 0-.69-.139-1.861-.902-2.206-.659-.298-1.664-.62-3.3-.04-3.752 1.942-6.711 5.88-7.798 7.995z" }
        }
    }
}

#[component]
pub fn CalendarIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "4", width: "18", height: "18", rx: "2", ry: "2" }
            line { x1: "16", x2: "16", y1: "2", y2: "6" }
            line { x1: "8", x2: "8", y1: "2", y2: "6" }
            line { x1: "3", x2: "21", y1: "10", y2: "10" }
        }
    }
}

#[component]
pub fn LinkedInIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 0 1-2.063-2.065 2.064 2.064 0 1 1 2.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
        }
    }
}

#[component]
pub fn MusicNoteIcon(class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class.unwrap_or_default(),
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M9 18V5l12-2v13" }
            circle { cx: "6", cy: "18", r: "3" }
            circle { cx: "18", cy: "16", r: "3" }
        }
    }
}
