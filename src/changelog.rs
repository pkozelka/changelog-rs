use std::collections::HashMap;
use std::io::Write;

use chrono::NaiveDate;

use crate::ChangeLogConfig;
use crate::changeset::{ChangeSet, ChangesetHeader};
use crate::ChgError;

impl ChangeLog {
    /// Finds missing items in the changelog.
///
/// The general assumptions are:
/// - `old` is manually updated and must be fully respected
/// - `new` is auto-generated content, and is rather a recommendation
/// - presence of items is based on presence of URLs (issues, PRs, other)
/// - `new` contains most records of `old` (but some may have been manually added or removed)
///
/// Situations:
/// * a) both changelogs have the same LATEST_RELEASE version
/// * b) `new` comes with one or more releases - then the `Unreleased` changeset is matched against first(oldest) additional release; it's good when there is an item prooving the match
/// * c) release histories are disjunct => error, cannot update
/// * d) LATEST_RELEASE of `old` is not present in new => error, cannot update
    pub fn sync_from(&mut self, new: &ChangeLog) -> Result<(), ChgError> {
        // bring missing (NEW) released items into OLD unreleased section, and move it to OLD releases
        // bring following comple
        // te releases omitted in OLD side (if any)
        // bring missing NEW unreleased items into OLD unreleased section

        let (old_rvs, _old_changeset) = self
            .releases
            .get(0)
            .expect("TODO: Old changeset has no release yet"); // TODO
        let (new_rvs, _new_changeset) = new
            .releases
            .get(0)
            .expect("TODO: New changeset has no release yet"); // TODO

        if old_rvs.version == new_rvs.version {
            // only sync new unreleased into old unreleased
            let mut old_unreleased = match &self.unreleased {
                None => ChangeSet { header: ChangesetHeader::Unreleased, items: vec![] },
                Some(_) => self.unreleased.take().unwrap(),
            };
            old_unreleased.sync_from(new.unreleased.as_ref().unwrap());
            self.unreleased = Some(old_unreleased);
        } else {
            // find all new changesets
            let mut newcs: Vec<ChangeSet> = new
                .releases
                .iter()
                .take_while(|changeset| {
                    match &changeset.header {
                        ChangesetHeader::Unreleased => true,
                        ChangesetHeader::Release(rvs) => {
                            rvs.version != old_rvs.version
                        }
                    }
                })
                .map(|changeset|*changeset)
                .collect();
            newcs.reverse();
            trace!("New changesets: {}", newcs.len());
            trace!("Existing changesets: {}", self.releases.len());

            // 1. old unreleased receives oldest new release
            let mut old_unreleased = match &self.unreleased {
                None => ChangeSet { header: ChangesetHeader::Unreleased, items: vec![] },
                Some(_) => self.unreleased.take().unwrap(),
            };
            let (new_release_header, new_release_changeset) = newcs.remove(0);
            trace!("1. syncing unreleased news from {}", new_release_header.version);
            old_unreleased.sync_from(new_release_changeset);

            // 2. old unreleased becomes release
            trace!("2. switching old unreleased to release: {}", new_release_header.version);
            self.releases.insert(0, (new_release_header.clone(), old_unreleased));
            // 3. other new releases are copied to old releases, keeping order
            for r in newcs {
                trace!("3. copying entire section {:?}", r.header);
                self.releases.insert(0, r.clone());
            }
            // 4. new unreleased is copied to old unreleased
            if let Some(unreleased) = &new.unreleased {
                trace!("4. adding new unreleased section ({} items)", unreleased.items.len());
                for item in &unreleased.items {
                    trace!("   * {:?}", item);
                }
                self.unreleased = new.unreleased.clone();
            }
        }
        trace!("Existing changesets: {}", self.releases.len());
        Ok(())
    }
}

/// Entire changelog.
#[derive(Debug)]
pub struct ChangeLog {
    pub meta: HashMap<String, String>,
    pub prolog: String,
    pub releases: Vec<ChangeSet>,
    pub epilog: String,
    pub config: ChangeLogConfig,
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

    pub fn release(tag: &str, timestamp: NaiveDate, yanked: bool) -> ChangesetHeader {
        let mut version = &tag[..];
        for c in version.chars() {
            if c.is_ascii_digit() {
                break;
            };
            version = &version[1..];
        }
        if version.is_empty() {
            ChangesetHeader::Unreleased
        } else {
            ChangesetHeader::Release(Self {
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
        for release in &self.releases {
            Self::print_markdown_items(out, &release)?;
        }
        Ok(())
    }

    fn print_markdown_items(out: &mut dyn Write, changes: &ChangeSet) -> std::io::Result<()> {
        match &changes.header {
            ChangesetHeader::Unreleased => {
                writeln!(out, "## Unreleased")?;
            }
            ChangesetHeader::Release(ver) => {
                let ts = ver.timestamp.to_string();
                writeln!(
                    out,
                    "## {} - {}{}",
                    ver.version,
                    &ts[0..10],
                    if ver.yanked { " [YANKED]" } else { "" }
                )?;
            }
        }
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
