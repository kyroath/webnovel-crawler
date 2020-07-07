#[derive(Debug, PartialEq)]
pub enum Source {
    Fanfiction,
    NotGiven,
}

impl std::default::Default for Source {
    fn default() -> Self {
        Source::NotGiven
    }
}

#[derive(Debug, Default)]
pub struct FictionInfo {
    pub host: Source,
    pub id: String,
    pub name: String,
    pub chapterCount: usize,
    pub wordCount: Option<usize>,
}

impl FictionInfo {
    pub fn parse_url_str(url: &str) -> Self {
        FictionInfo::parse_url(url.to_string())
    }

    pub fn parse_url(url: String) -> Self {
        FictionInfo {
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_info() {
        let t = FictionInfo {
            ..Default::default()
        };

        let parsed = FictionInfo::parse_url_str("test");

        assert_eq!(parsed.host, t.host);
        assert_eq!(parsed.id, t.id);
        assert_eq!(parsed.name, t.name);
        assert_eq!(parsed.chapterCount, t.chapterCount);
        assert_eq!(parsed.wordCount, t.wordCount);
    }

    #[test]
    fn parse_info_string() {
        let t = FictionInfo {
            ..Default::default()
        };

        let parsed = FictionInfo::parse_url(String::from("test"));

        assert_eq!(parsed.host, t.host);
        assert_eq!(parsed.id, t.id);
        assert_eq!(parsed.name, t.name);
        assert_eq!(parsed.chapterCount, t.chapterCount);
        assert_eq!(parsed.wordCount, t.wordCount);
    }
}
