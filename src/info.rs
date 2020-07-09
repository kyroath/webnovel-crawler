use std::convert::From;
use url::{ParseError, Url};

#[derive(Debug, PartialEq)]
pub enum Source {
    Fanfiction,
    NotGiven,
}

impl Source {
    fn parse(url: &Url) -> Self {
        info!("Parsing URL to get the host");
        let host = url.host().unwrap().to_string();

        info!("Host string extracted, matching it from source list");
        match host.as_str() {
            "www.fanfiction.net" | "fanfiction.net" => Source::Fanfiction,
            _ => Source::NotGiven,
        }
    }
}

impl std::default::Default for Source {
    fn default() -> Self {
        Source::NotGiven
    }
}

#[derive(Debug, Default)]
pub struct FictionInfo {
    url: Option<Url>,
    pub source: Source,
    pub id: String,
    pub name: String,
    pub chapter_count: usize,
    pub word_count: Option<usize>,
}

impl FictionInfo {
    pub fn new_default() -> Self {
        FictionInfo {
            ..Default::default()
        }
    }
}

impl FictionInfo {
    pub fn from_url_str(url: &str) -> Self {
        FictionInfo::from_url(url.to_string())
    }

    pub fn from_url(url: String) -> Self {
        let mut info = FictionInfo::new_default();
        let url = Url::parse(url.clone().as_str())
            .or_else(|err| match err {
                ParseError::RelativeUrlWithoutBase => {
                    debug!(
                        "URL is relative, adding {} in front",
                        console::style("http://").magenta()
                    );
                    let new_url = format!("http://{}", url.clone());
                    Url::parse(new_url.as_str())
                }
                _ => Err(err),
            })
            .unwrap();

        info.url = Some(url);
        info.parse();

        info
    }
}

impl FictionInfo {
    pub fn parse(&mut self) {
        self.source = Source::parse(&self.url.clone().unwrap());
        self.parse_id();
        self.fetch_metadata();
    }

    fn parse_id(&mut self) {
        match self.source {
            Source::Fanfiction => {
                self.id = self
                    .url
                    .clone()
                    .unwrap()
                    .path()
                    .to_string()
                    .split("/")
                    .collect::<Vec<&str>>()[2]
                    .to_string();
            }
            _ => self.id = String::from(""),
        }
    }

    pub fn fetch_metadata(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_info() {
        let t = FictionInfo {
            ..Default::default()
        };

        let parsed = FictionInfo::from_url(String::from("test"));

        assert_eq!(parsed.source, t.source);
        assert_eq!(parsed.id, t.id);
        assert_eq!(parsed.name, t.name);
        assert_eq!(parsed.chapter_count, t.chapter_count);
        assert_eq!(parsed.word_count, t.word_count);
    }

    #[test]
    fn parse_info_str() {
        let t = FictionInfo {
            ..Default::default()
        };

        let parsed = FictionInfo::from_url_str("test");

        assert_eq!(parsed.source, t.source);
        assert_eq!(parsed.id, t.id);
        assert_eq!(parsed.name, t.name);
        assert_eq!(parsed.chapter_count, t.chapter_count);
        assert_eq!(parsed.word_count, t.word_count);
    }

    #[test]
    fn parse_info_fanfiction() {
        let parsed = FictionInfo::from_url(String::from("http://fanfiction.net/s/123456"));

        assert_eq!(parsed.source, Source::Fanfiction);
        assert_eq!(parsed.id, String::from("123456"));
    }
}
