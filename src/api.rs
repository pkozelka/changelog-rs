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
        date: String,
    },
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
pub enum ChangeType {
    Other,
    Added,
    Fixed,
    Changed,
    Deprecated,
    Removed,
    Refactored,
}
