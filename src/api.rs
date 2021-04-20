use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

/// Entire changelog.
pub struct ChangeLog {
    pub meta: HashMap<String, String>,
    pub prolog: String,
    pub versions: Vec<ChangeSet>,
    pub epilog: String,
}

/// Container of changes related to one version, either released or unreleased.
pub struct ChangeSet {
    pub version_spec: VersionSpec,
    pub items: Vec<ChangeItem>,
}

/// Supports unreleased and released versions
pub enum VersionSpec {
    /// Unreleased section. In workflows tracking multiple major versions, the major version can be supplied.
    Unreleased {
        major: Option<String>,
        branch: Option<String>,
    },
    /// Released section.
    Release {
        version: String,
        tag: String,
        /// TODO consider using NaiveDate[Time] here
        timestamp: DateTime<FixedOffset>,
        /// yanked are versions that we had to withdraw due to a significant problem found after release
        yanked: bool,
    },
}

impl VersionSpec {
    pub fn unreleased() -> Self {
        Self::Unreleased {
            major: None,
            branch: None,
        }
    }

    pub fn release_tagged(tag: &str, version: &str, timestamp: DateTime<FixedOffset>, yanked: bool) -> Self {
        Self::Release {
            version: version.to_string(),
            tag: tag.to_string(),
            timestamp,
            yanked,
        }
    }

    pub fn release(tag: &str, timestamp: DateTime<FixedOffset>, yanked: bool) -> Self {
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
