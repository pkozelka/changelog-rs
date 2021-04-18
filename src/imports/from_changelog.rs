use std::io::BufRead;
use std::io::Result;

use chrono::{DateTime, FixedOffset, NaiveDate};
use regex::Regex;

use crate::api::{ChangeItem, ChangeLog, ChangeType, VersionSpec};
use crate::builder::ChangeLogBuilder;

enum ParserState {
    Prolog,
    Section,
    Epilog,
}

pub fn parse(reader: &mut dyn BufRead) -> Result<ChangeLog> {
    let mut builder = ChangeLogBuilder::new();
    let lines = reader.lines();
    let mut state = ParserState::Prolog;
    // read prolog
    for line in lines {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        match state {
            ParserState::Prolog | ParserState::Section => {
                if line.starts_with("## ") {
                    builder.section(VersionSpec::parse_section_header(&line[3..])?);
                    state = ParserState::Section;
                    continue;
                }
            }
            ParserState::Epilog => { /* until the EOF */ }
        }
        match state {
            ParserState::Section => {
                let change_item = ChangeItem::parse_item(&line)?;
                match change_item {
                    None => {
                        builder.note(&line)?;
                        state = ParserState::Epilog;
                    }
                    Some(change_item) => {
                        builder.item(change_item)?;
                    }
                }
            }
            _ => {
                builder.note(&line)?;
            }
        }
    }
    Ok(builder.build())
}

impl ChangeItem {
    fn parse_item(s: &str) -> Result<Option<Self>> {
        if s.starts_with("- ") || s.starts_with("* ") {
            let r = Regex::new(
                "((?P<refs>.*?):)?\\s*(?P<compo>\\[\\S+])?\\s*(?P<text>.*)/(?P<authors>.*)$",
            )
            .unwrap();
            let s = &s[2..];
            let captures = match r.captures(s) {
                None => panic!("Invalid item line: '{}'", s),
                Some(c) => c,
            };
            let refs: Vec<String> = match captures.name("refs") {
                None => Vec::new(),
                Some(refs) => refs
                    .as_str()
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect(),
            };
            let component = captures
                .name("compo")
                .map(|m| {
                    let s = m.as_str();
                    s[1..s.len() - 1].to_string()
                })
                .unwrap_or("".to_string());
            let text = captures.name("text").unwrap().as_str().trim().to_string();
            let authors: Vec<String> = captures
                .name("authors")
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.trim().to_string())
                .collect();
            let chgi = ChangeItem {
                refs,
                change_type: ChangeType::Other,
                component,
                text,
                authors,
            };
            Ok(Some(chgi))
        } else {
            Ok(None)
        }
    }
}

impl VersionSpec {
    fn parse_section_header(s: &str) -> Result<Self> {
        if s == "Unreleased" {
            Ok(VersionSpec::Unreleased {
                major: None,
                branch: None,
            })
        } else {
            let r = Regex::new(
                "(?P<version>[0-9.]+)\\s+-\\s+(?P<timestamp>\\d+-\\d+-\\d+)\\s*(?P<more>.*)$",
            )
            .unwrap();
            let captures = r.captures(s).unwrap();
            let version = captures.name("version").unwrap().as_str();
            let timestamp = captures.name("timestamp").unwrap().as_str();
            let timestamp = NaiveDate::parse_from_str(timestamp, "%Y-%m-%d")
                .unwrap()
                .and_hms(0, 0, 0);
            let timestamp = DateTime::<FixedOffset>::from_utc(timestamp, FixedOffset::west(0));
            let more = captures.name("more").unwrap().as_str();
            let yanked = more.to_ascii_uppercase().contains("YANKED");
            Ok(VersionSpec::Release {
                version: version.to_string(),
                tag: "".to_string(),
                timestamp,
                yanked,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset, NaiveDate};

    use crate::api::{ChangeItem, ChangeType, VersionSpec};

    #[test]
    fn test_parse_section_header_unreleased() {
        let header = VersionSpec::parse_section_header("Unreleased").unwrap();
        match header {
            VersionSpec::Unreleased { major, branch } => {
                assert_eq!(major, None);
                assert_eq!(branch, None);
            }
            VersionSpec::Release { .. } => {
                panic!("Unreleased expected here")
            }
        }
    }

    #[test]
    fn test_parse_section_header_released() {
        let header = VersionSpec::parse_section_header("2.5.6 - 2020-12-10").unwrap();
        match header {
            VersionSpec::Unreleased { .. } => {
                panic!("Release expected here")
            }
            VersionSpec::Release {
                version,
                tag,
                timestamp,
                yanked,
            } => {
                assert_eq!(version, "2.5.6", "version");
                assert_eq!(tag, "", "tag");
                assert_eq!(yanked, false, "yanked");
                let naive = NaiveDate::from_ymd(2020, 12, 10).and_hms(0, 0, 0);
                let ts = DateTime::<FixedOffset>::from_utc(naive, FixedOffset::west(0));
                assert_eq!(timestamp, ts);
            }
        }
    }

    #[test]
    fn test_parse_item() {
        let item = ChangeItem::parse_item(
            "- PR#629: [java] parse the UUID of mojo. close #628 / Qiang Kou",
        )
        .unwrap();
        assert_eq!(item.is_some(), true, "No section item was parsed");
        let item = item.unwrap();
        assert_eq!(item.refs.len(), 1, "Refs: {:?}", item.refs);
        assert_eq!(item.refs[0], "PR#629", "Refs: {:?}", item.refs);
        assert_eq!(item.component, "java");
        assert_eq!(item.change_type, ChangeType::Other);
        assert_eq!(item.text, "parse the UUID of mojo. close #628");
        assert_eq!(item.authors.len(), 1, "Authors: {:?}", item.authors);
        assert_eq!(item.authors[0], "Qiang Kou", "Authors: {:?}", item.authors);
    }
}
