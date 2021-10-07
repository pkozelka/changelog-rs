use crate::api::ChangeItem;
use std::collections::HashSet;

/// Container of changes related to one version, either released or unreleased.
#[derive(Debug, Clone)]
pub struct ChangeSet {
    pub items: Vec<ChangeItem>,
}

impl ChangeSet {
    pub(crate) fn sync_from(&mut self, from: &ChangeSet) {
        // gather all urls on `this` side
        let mut this_urls = HashSet::new();
        {
            for item in &self.items {
                for href in &item.refs {
                    this_urls.insert(href.clone());
                }
            }
        }
        // go through `from` items; for each with missing url on `this`, add it
        for item in from.items.iter().rev() {
            if item.refs.is_empty() {
                //TODO somehow, check if that item already exists
                self.items.insert(0, item.clone());
            }
            for href in &item.refs {
                if !this_urls.contains(href) {
                    self.items.insert(0, item.clone());
                    //TODO also save a command!
                    for r in &item.refs { this_urls.insert(r.clone()); }
                    break;
                }
            }
        }
    }
}
