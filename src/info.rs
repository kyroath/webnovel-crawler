pub enum Source {
    Fanfiction,
    NotGiven,
}

impl std::default::Default for Source {
    fn default() -> Self {
        Source::NotGiven
    }
}

#[derive(Default)]
pub struct FictionInfo {
    pub host: Source,
    pub id: String,
    pub name: String,
    pub chapterCount: usize,
    pub wordCount: Option<usize>,
}

impl FictionInfo {
    pub fn parse_url(url: String) -> Self {
        FictionInfo {
            ..Default::default()
        }
    }
}
