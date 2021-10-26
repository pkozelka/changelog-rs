use std::io::{Error, ErrorKind, Result};

use crate::{ChangeItem, ChangeLog, ChangeLogConfig, ReleaseHeader};
use crate::changeset::{ChangeSet, ChangesetHeader};

/// Stateful helper for building changelog while parsing it from a file.
/// Line parsing is assumed and best supported.
pub struct ChangeLogBuilder {
    current_release: Option<ReleaseHeader>,
    current_section: Option<ChangeSet>,
    changelog: ChangeLog,
}

impl ChangeLogBuilder {
    pub fn new(config: ChangeLogConfig) -> Self {
        Self {
            current_release: None,
            current_section: None,
            changelog: ChangeLog {
                meta: Default::default(),
                prolog: "".to_string(),
                releases: vec![],
                epilog: "".to_string(),
                config: config.clone(),
            },
        }
    }

    pub fn section(&mut self, header: ChangesetHeader) {
        self.current_section_close();
        self.current_release = None; //TODO Eliminate!!!
        self.current_section = Some(ChangeSet { header, items: vec![] });
    }

    fn current_section_close(&mut self) {
        if let Some(current) = self.current_section.take() {
            self.changelog.releases.push(current);
        }
    }

    pub fn item(&mut self, item: ChangeItem) -> Result<()> {
        let section = match &mut self.current_section {
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "No section precedes this item",
                ))
            }
            Some(current) => current,
        };
        section.items.push(item);
        Ok(())
    }

    pub fn note(&mut self, line: &str) -> Result<()> {
        self.current_section_close();
        if self.changelog.releases.is_empty() {
            self.changelog.prolog.add_line(line);
        } else {
            self.changelog.epilog.add_line(line);
        }
        Ok(())
    }

    pub fn build(mut self) -> ChangeLog {
        self.current_section_close();
        self.changelog
    }
}

trait MyOptString {
    fn add_line(&mut self, line: &str);
}

impl MyOptString for String {
    fn add_line(&mut self, line: &str) {
        self.push_str("\n");
        self.push_str(line);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ChangeItem, ChangeLogConfig, ChangeType};
    use crate::builder::ChangeLogBuilder;
    use crate::changeset::ChangesetHeader;

    #[test]
    fn usage_primitives() {
        let mut builder = ChangeLogBuilder::new(ChangeLogConfig::default());
        // prolog
        builder.note("hello").unwrap();
        builder.note("Hello").unwrap();
        builder.section(ChangesetHeader::Unreleased);
        builder
            .item(ChangeItem {
                refs: vec![],
                change_type: ChangeType::Other,
                component: "".to_string(),
                text: "".to_string(),
                authors: vec![],
            })
            .unwrap();
        // epilog
        builder.note("World").unwrap();
        builder.note("world").unwrap();

        let changelog = builder.build();
        println!("prolog: {}", changelog.prolog);
        println!("epilog: {}", changelog.epilog);
        assert!(!changelog.releases[0].header.is_release());
    }
}
