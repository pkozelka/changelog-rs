use crate::api::{ChangeItem, ChangeLog, ChangeSet, VersionSpec};

use std::io::{Error, ErrorKind, Result};
use crate::ChangeLogConfig;

/// Stateful helper for building changelog while parsing it from a file.
/// Line parsing is assumed and best supported.
pub struct ChangeLogBuilder {
    prolog: Option<String>,
    epilog: Option<String>,
    current_section: Option<ChangeSet>,
    sections: Vec<ChangeSet>,
    pub config: ChangeLogConfig,
}

impl ChangeLogBuilder {
    pub fn new() -> Self {
        Self {
            prolog: None,
            epilog: None,
            current_section: None,
            sections: vec![],
            config: ChangeLogConfig::default(),
        }
    }

    pub fn section(&mut self, header: VersionSpec) {
        self.current_section_close();
        self.current_section = Some(ChangeSet {
            version_spec: header,
            items: vec![],
        });
    }

    fn current_section_close(&mut self) {
        // let current = std::mem::replace(&mut self.current_section, None);
        if let Some(current) = self.current_section.take() {
            self.sections.push(current);
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
        if self.sections.is_empty() {
            self.prolog.add_line(line);
        } else {
            self.epilog.add_line(line);
        }
        Ok(())
    }

    pub fn build(mut self) -> ChangeLog {
        self.current_section_close();
        ChangeLog {
            meta: Default::default(),
            prolog: self.prolog.unwrap_or("".to_string()),
            versions: self.sections,
            epilog: self.epilog.unwrap_or("".to_string()),
            config: self.config,
        }
    }
}

trait MyOptString {
    fn add_line(&mut self, line: &str);
}

impl MyOptString for Option<String> {
    fn add_line(&mut self, line: &str) {
        match self {
            None => {
                self.replace(line.to_string());
            }
            Some(text) => {
                text.push_str("\n");
                text.push_str(line);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::VersionSpec::Unreleased;
    use crate::api::{ChangeItem, ChangeType};
    use crate::builder::ChangeLogBuilder;

    #[test]
    fn usage_primitives() {
        let mut builder = ChangeLogBuilder::new();
        // prolog
        builder.note("hello").unwrap();
        builder.note("Hello").unwrap();
        builder.section(Unreleased {
            major: None,
            branch: None,
        });
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
        assert_eq!(changelog.versions.len(), 1);
    }
}
