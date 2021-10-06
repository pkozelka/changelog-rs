use std::collections::HashSet;

use crate::{ChangeLog, ChgError};
use crate::api::{ChangeSet, ReleaseHeader};

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
                None => ChangeSet { items: vec![] },
                Some(_) => self.unreleased.take().unwrap(),
            };
            changeset_sync(&mut old_unreleased, new.unreleased.as_ref().unwrap());
            self.unreleased = Some(old_unreleased);
        } else {
            // find all new changesets
            let mut newcs: Vec<&(ReleaseHeader, ChangeSet)> = new
                .releases
                .iter()
                .take_while(|(rvs, _)| rvs.version != old_rvs.version)
                .collect();
            newcs.reverse();
            trace!("New changesets: {}", newcs.len());
            trace!("Existing changesets: {}", self.releases.len());

            // 1. old unreleased receives oldest new release
            let mut old_unreleased = match &self.unreleased {
                None => ChangeSet { items: vec![] },
                Some(_) => self.unreleased.take().unwrap(),
            };
            let (new_release_header, new_release_changeset) = newcs.remove(0);
            trace!("1. syncing unreleased news from {}", new_release_header.version);
            changeset_sync(&mut old_unreleased, new_release_changeset);

            // 2. old unreleased becomes release
            trace!("2. switching old unreleased to release: {}", new_release_header.version);
            self.releases.insert(0, (new_release_header.clone(), old_unreleased));
            // 3. other new releases are copied to old releases, keeping order
            for r in newcs {
                trace!("3. copying entire section {}", r.0.version);
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

fn changeset_sync(this: &mut ChangeSet, from: &ChangeSet) {
    // gather all urls on `this` side
    let mut this_urls = HashSet::new();
    {
        for item in &this.items {
            for href in &item.refs {
                this_urls.insert(href.clone());
            }
        }
    }
    // go through `from` items; for each with missing url on `this`, add it
    for item in from.items.iter().rev() {
        if item.refs.is_empty() {
            //TODO somehow, check if that item already exists
            this.items.insert(0, item.clone());
            trace!("adding '{:?}' because it has no urls", item);
        }
        for href in &item.refs {
            if !this_urls.contains(href) {
                this.items.insert(0, item.clone());
                //TODO also save a command!
                trace!("adding '{:?}' because of {}", item, href);
                for r in &item.refs { this_urls.insert(r.clone()); }
                break;
            }
        }
    }
}
