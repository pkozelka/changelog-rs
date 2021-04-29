use std::collections::HashMap;
use std::io::Write;

use chrono::NaiveDate;

use crate::ChangeLogConfig;

/// Entire changelog.
pub struct ChangeLog {
    pub meta: HashMap<String, String>,
    pub prolog: String,
    pub unreleased: Option<ChangeSet>,
    pub releases: Vec<(ReleaseHeader, ChangeSet)>,
    pub epilog: String,
    pub config: ChangeLogConfig,
}

/// Container of changes related to one version, either released or unreleased.
#[derive(Debug, Clone)]
pub struct ChangeSet {
    pub items: Vec<ChangeItem>,
}

#[derive(Debug, Clone)]
pub struct ReleaseHeader {
    pub version: String,
    pub tag: String,
    /// date of the release
    pub timestamp: NaiveDate,
    /// yanked are versions that we had to unpublish, usually due to a significant problem found after release
    pub yanked: bool,
}

impl ReleaseHeader {
    pub fn release_tagged(tag: &str, version: &str, timestamp: NaiveDate, yanked: bool) -> Self {
        Self {
            version: version.to_string(),
            tag: tag.to_string(),
            timestamp,
            yanked,
        }
    }

    pub fn release(tag: &str, timestamp: NaiveDate, yanked: bool) -> Option<Self> {
        let mut version = &tag[..];
        for c in version.chars() {
            if c.is_ascii_digit() {
                break;
            };
            version = &version[1..];
        }
        if version.is_empty() {
            None
        } else {
            Some(Self {
                version: version.to_string(),
                tag: tag.to_string(),
                timestamp,
                yanked,
            })
        }
    }
}

/// One change it the release.
#[derive(Clone, Debug)]
pub struct ChangeItem {
    pub refs: Vec<String>,
    pub change_type: ChangeType,
    pub component: String,
    pub text: String,
    pub authors: Vec<String>,
}

/// Type of the change
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum ChangeType {
    Other,
    Added,
    Fixed,
    Changed,
    Deprecated,
    Removed,
    Refactored,
}

impl ChangeLog {
    pub fn print_markdown(&self, out: &mut dyn Write) -> std::io::Result<()> {
        if let Some(unreleased) = &self.unreleased {
            writeln!(out, "## Unreleased")?;
            Self::print_markdown_items(out, &unreleased)?;
        }

        for (ver, release) in &self.releases {
            let ts = ver.timestamp.to_string();
            writeln!(
                out,
                "## {} - {}{}",
                ver.version,
                &ts[0..10],
                if ver.yanked { " [YANKED]" } else { "" }
            )?;
            Self::print_markdown_items(out, &release)?;
        }
        Ok(())
    }

    fn print_markdown_items(out: &mut dyn Write, changes: &ChangeSet) -> std::io::Result<()> {
        if !changes.items.is_empty() {
            writeln!(out)?;
            for item in &changes.items {
                write!(out, "- ")?;
                if !item.refs.is_empty() {
                    write!(out, "{}: ", item.refs.join(", "))?;
                }
                if !item.component.is_empty() {
                    write!(out, "[{}] ", item.component)?;
                }
                writeln!(out, "{} / {}", item.text, item.authors.join(", "))?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}
