use std::collections::HashMap;
use std::io::Write;

use chrono::NaiveDate;

use crate::ChangeLogConfig;

/// Entire changelog.
pub struct ChangeLog {
    pub meta: HashMap<String, String>,
    pub prolog: String,
    pub versions: Vec<ChangeSet>,
    pub epilog: String,
    pub config: ChangeLogConfig,
}

/// Container of changes related to one version, either released or unreleased.
#[derive(Debug)]
pub struct ChangeSet {
    pub version_spec: VersionSpec,
    pub items: Vec<ChangeItem>,
}

/// Supports unreleased and released versions
#[derive(Debug)]
pub enum VersionSpec {
    /// Unreleased section.
    Unreleased,
    /// Released section.
    Release {
        version: String,
        tag: String,
        /// date of the release
        timestamp: NaiveDate,
        /// yanked are versions that we had to withdraw due to a significant problem found after release
        yanked: bool,
    },
}

impl VersionSpec {
    pub fn unreleased() -> Self {
        Self::Unreleased {
        }
    }

    pub fn release_tagged(
        tag: &str,
        version: &str,
        timestamp: NaiveDate,
        yanked: bool,
    ) -> Self {
        Self::Release {
            version: version.to_string(),
            tag: tag.to_string(),
            timestamp,
            yanked,
        }
    }

    pub fn release(tag: &str, timestamp: NaiveDate, yanked: bool) -> Self {
        let mut version = &tag[..];
        for c in version.chars() {
            if c.is_ascii_digit() {
                break;
            };
            version = &version[1..];
        }
        Self::Release {
            version: version.to_string(),
            tag: tag.to_string(),
            timestamp,
            yanked,
        }
    }
}

/// One change it the release.
#[derive(Debug)]
pub struct ChangeItem {
    pub refs: Vec<String>,
    pub change_type: ChangeType,
    pub component: String,
    pub text: String,
    pub authors: Vec<String>,
}

/// Type of the change
#[derive(Eq, PartialEq, Debug)]
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
    pub fn to_markdown(&self, out: &mut dyn Write) -> std::io::Result<()> {
        for release in &self.versions {
            match &release.version_spec {
                VersionSpec::Unreleased { .. } => {
                    writeln!(out, "## Unreleased")?;
                }
                VersionSpec::Release {
                    version,
                    tag: _,
                    timestamp,
                    yanked,
                } => {
                    let ts = timestamp.to_string();
                    writeln!(
                        out,
                        "## {} - {}{}",
                        version,
                        &ts[0..10],
                        if *yanked { " [YANKED]" } else { "" }
                    )?;
                }
            }
            if !release.items.is_empty() {
                writeln!(out)?;
                for item in &release.items {
                    write!(out, "- ")?;
                    if !item.refs.is_empty() {
                        write!(out, "{}: ", item.refs.join(", "))?;
                    }
                    writeln!(out, "{} / {}", item.text, item.authors.join(", "))?;
                }
                writeln!(out)?;
            }
        }
        Ok(())
    }
}
