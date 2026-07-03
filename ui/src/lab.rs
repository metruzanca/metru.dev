#[derive(Clone, PartialEq)]
pub struct LabMeta {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}
