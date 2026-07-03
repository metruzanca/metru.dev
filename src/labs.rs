use dioxus::prelude::*;
use ui::LabMeta;

#[derive(Clone, Copy)]
pub struct LabInfo {
    pub meta: &'static LabMeta,
    pub render: fn() -> Element,
}

impl PartialEq for LabInfo {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.meta, other.meta)
    }
}

pub static LABS: &[LabInfo] = &[
    LabInfo {
        meta: &lab_word_count::META,
        render: lab_word_count::App,
    },
];
