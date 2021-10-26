use chrono::NaiveDate;
use regex::Regex;

use crate::{ChangeItem, ChangeLog, ChangeLogConfig, ChangeType, ReleaseHeader};
use crate::builder::ChangeLogBuilder;
use crate::error::ChgError;
use std::io::{BufRead, Cursor};
use crate::changeset::ChangesetHeader;

enum ParserState {
    Prolog,
    Section,
    Epilog,
}

impl ChangeLog {
    pub fn import_markdown(text: &str) -> Result<ChangeLog, ChgError> {
        let config = ChangeLogConfig::parse_embedded(&text)?;
        let mut cursor = Cursor::new(text);
        let builder = ChangeLogBuilder::import(&mut cursor, config)?;
        Ok(builder.build())
    }
}

impl ChangeLogBuilder {
    pub fn import(reader: impl BufRead, config: ChangeLogConfig) -> Result<Self, ChgError> {
        let mut this = Self::new(config);
        this.parse(reader)?;
        Ok(this)
    }

    fn parse(&mut self, reader: impl BufRead) -> Result<(), ChgError> {
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
                        self.section(ChangesetHeader::parse_section_header(&line[3..])?);
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
                            self.note(&line)?;
                            state = ParserState::Epilog;
                        }
                        Some(change_item) => {
                            self.item(change_item)?;
                        }
                    }
                }
                _ => {
                    self.note(&line)?;
                }
            }
        }
        Ok(())
    }
}

impl ChangeItem {
    fn parse_item(s: &str) -> Result<Option<Self>, ChgError> {
        if s.starts_with("- ") || s.starts_with("* ") {
            let r = Regex::new(
                "((?P<refs>\\w.*?):)?\\s*(?P<compo>\\[\\S+])?\\s*(?P<text>.*)/(?P<authors>.*)$",
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

impl ChangesetHeader {
    /// Parses section header string into a VersionSpec.
    ///
    /// Following formats are acceptable:
    ///
    /// * `1.2.123 - 2020-04-20`
    /// * `1.2.3-1 2020-04-20`
    /// * `1.2.333 2020-04-20 yanked`
    /// * `1.2.3.b5.c7-a 2020-04-20 yanked`
    /// * `Unreleased`
    fn parse_section_header(s: &str) -> Result<Self, ChgError> {
        let s = s.trim();
        if s.to_ascii_lowercase() == "unreleased" {
            Ok(ChangesetHeader::Unreleased)
        } else {
            let mut section_tokens = s.trim().split(' ');

            // version (required)
            let version = section_tokens
                .next()
                .ok_or_else(|| ChgError::MissingVersionDateSeparator(s.to_owned()))?;
            // - validate version string
            if version.is_empty() {
                return Err(ChgError::InvalidVersionID(version.to_owned(), s.to_owned()));
            } else if let Some(c) = version.chars().next() {
                // first character of the version must be a digit
                if !c.is_ascii_digit() {
                    return Err(ChgError::InvalidVersionID(version.to_owned(), s.to_owned()));
                }
            }
            // separator (optional)
            let sep = section_tokens
                .next()
                .ok_or_else(|| ChgError::MissingVersionDateSeparator(s.to_owned()))?
                .trim();

            // timestamp (required)
            let timestamp = if sep == "-" {
                section_tokens
                    .next()
                    .ok_or_else(|| ChgError::MissingTimestamp(s.to_owned()))?
                    .trim()
            } else {
                sep
            };

            // - parse timestamp
            let r = Regex::new("(?P<timestamp>\\d+-\\d+-\\d+)$").unwrap();
            let timestamp = r.captures(timestamp)
                .and_then(|captures| captures.name("timestamp"))
                .and_then(|m| Some(m.as_str()))
                .ok_or(ChgError::InvalidTimestamp(timestamp.to_owned(), s.to_owned()))?;
            let timestamp = NaiveDate::parse_from_str(timestamp, "%Y-%m-%d")
                .or_else(|e| Err(ChgError::InvalidTimestamp(s.to_owned(), e.to_string())))?;

            // yanked
            let yanked = if let Some(more) = section_tokens.next() {
                more.to_ascii_uppercase().contains("YANKED")
            } else {
                false
            };

            //
            Ok(ChangesetHeader::Release(ReleaseHeader {
                version: version.to_string(),
                tag: "".to_string(),
                timestamp,
                yanked,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{ChangeItem, ChangeType, ReleaseHeader};
    use crate::changeset::ChangesetHeader;

    #[test]
    fn test_parse_section_header_unreleased() {
        let header = ChangesetHeader::parse_section_header("Unreleased").unwrap();
        assert!(!header.is_release(), "Unreleased expected here");
    }

    #[test]
    fn test_parse_section_header_released() {
        let header = ChangesetHeader::parse_section_header("2.5.6 - 2020-12-10").unwrap();
        match header {
            ChangesetHeader::Unreleased => panic!("Release expected here"),
            ChangesetHeader::Release(ReleaseHeader {
                version,
                tag,
                timestamp,
                yanked,
            }) => {
                assert_eq!(version, "2.5.6", "version");
                assert_eq!(tag, "", "tag");
                assert_eq!(yanked, false, "yanked");
                let ts = NaiveDate::from_ymd(2020, 12, 10);
                assert_eq!(timestamp, ts);
            }
        }
    }

    #[test]
    fn test_parse_section_header_released_noseparator_yanked() {
        let header = ChangesetHeader::parse_section_header("1.22.333-alpha-1 2021-04-20 YANKED").unwrap();
        match header {
            ChangesetHeader::Unreleased => panic!("Release expected here"),
            ChangesetHeader::Release(ReleaseHeader {
                version,
                tag,
                timestamp,
                yanked,
            }) => {
                assert_eq!(version, "1.22.333-alpha-1", "version");
                assert_eq!(tag, "", "tag");
                assert_eq!(yanked, true, "yanked");
                let ts = NaiveDate::from_ymd(2021, 04, 20);
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

    #[test]
    fn test_parse_item_with_fire() {
        let item = ChangeItem::parse_item(
            "- [HOTFIX] :fire: Wrong grep expression in our Jenkinsfile / mmalohlava",
        )
        .unwrap();
        println!("Parsed item: {:?}", item);
        assert_eq!(item.is_some(), true, "No section item was parsed");
        let item = item.unwrap();
        assert_eq!(item.refs.len(), 0, "Refs: {:?}", item.refs);
        assert_eq!(item.component, "HOTFIX", "component");
        assert_eq!(item.change_type, ChangeType::Other);
        assert_eq!(item.text, ":fire: Wrong grep expression in our Jenkinsfile");
        assert_eq!(item.authors.len(), 1, "Author count: {:?}", item.authors);
        assert_eq!(item.authors[0], "mmalohlava", "Authors: {:?}", item.authors);
    }
}
