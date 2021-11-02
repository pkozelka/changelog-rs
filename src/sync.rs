use std::collections::HashMap;

use crate::{ChangeSet, ChgError};
use crate::changeset::ChangesetHeader::*;

struct MetaInfo {
    /// number of unreleased sections at the beginning - can be only `0` or `1`; we don't use boolean for convenience
    unreleased: usize,
    /// number of unshared sections
    unshared_count: usize,
    /// index of newest shared section (we ignore all the following in the sync)
    shared_index: Option<usize>,
}

pub(super) fn sync_one_from(this: &mut Vec<ChangeSet>, newcs: &[ChangeSet]) -> Result<bool, ChgError> {
    let (old, new) = comparison_meta(this, newcs);

    match (old.shared_index, new.shared_index) {
        (None, None) => { // no releases are shared:
            // - only acceptable if no release exists in old (when we start writing changelog)
            // - otherwise, it is probably a changelog from different project
            if old.unshared_count > 0 {
                return Err(ChgError::Other("No shared release; changelog is probably from a different project".to_string()))
            }
            if new.unshared_count == 0 {
                return Err(ChgError::Other("There is nothing to synchronize".to_string()))
            }
            let new_oldest = &newcs[newcs.len() - 1];
            match this.as_mut_slice() {
                [.., ref mut oldest] => {
                    log::debug!("Updating unreleased block with {} items", oldest.items.len());
                    oldest.sync_from(newcs.last().unwrap());
                }
                _ => {
                    // syncing first version
                    this.push(new_oldest.clone());
                }
            }
        }
        (Some(old_shared_index), Some(new_shared_index)) => {
            match &this[old_shared_index].header {
                Unreleased => unreachable!(),
                Release(shared_rh) => {
                    log::debug!("new changelog shares some history with us (since version {} from {})", shared_rh.version, shared_rh.timestamp);
                }
            }
            if old.unshared_count > 0 {
                return Err(ChgError::Other(format!("Existing changelog diverges in {} releases", old.unshared_count)));
            }

            // take latest unshared from new and clone/sync it to this
            if new.unshared_count == 0 {
                if new.unreleased == 0 {
                    // nothing to synchronize
                    return Ok(false);
                }
                if old.unreleased == 0 {
                    // new has unreleased section, but old does not
                    let from = &newcs[0];
                    this.insert(0, from.clone());
                } else {
                    // both have unreleased section
                }
            } else {
                let from = &newcs[new_shared_index - 1];
                this.insert(0, from.clone());
            }
        }
        _ => unreachable!()
    }
    Ok(true)
}


/// Extracts comparison metadata, ie. anything important for synchronization
fn comparison_meta(left: &[ChangeSet], right: &[ChangeSet]) -> (MetaInfo, MetaInfo) {
    let (_, left_unreleased) = scan_versions(left);
    let (right_map, right_unreleased) = scan_versions(right);
    for (left_index, lcs) in left.iter().enumerate() {
        if let Release(rh) = &lcs.header {
            if let Some(right_index) = right_map.get(rh.version.as_str()) {
                let left_result = MetaInfo {
                    unreleased: left_unreleased,
                    unshared_count: left_index - left_unreleased,
                    shared_index: Some(left_index),
                };
                let right_result = MetaInfo {
                    unreleased: right_unreleased,
                    unshared_count: right_index - right_unreleased,
                    shared_index: Some(*right_index),
                };
                return (left_result, right_result);
            }
        }
    }
    // no shared part
    let left_result = MetaInfo {
        unreleased: left_unreleased,
        unshared_count: left.len() - left_unreleased,
        shared_index: None,
    };
    let right_result = MetaInfo {
        unreleased: right_unreleased,
        unshared_count: right.len() - right_unreleased,
        shared_index: None,
    };
    (left_result, right_result)
}

fn scan_versions(a: &[ChangeSet]) -> (HashMap<&str, usize>, usize) {
    let mut map = HashMap::new();
    let mut unreleased_heading = 0;
    for (index, cs) in a.iter().enumerate() {
        match &cs.header {
            Unreleased if index == 0 => {
                unreleased_heading = 1;
            },
            Unreleased => {
                log::error!("unreleased section found at index {}", index);
            },
            Release(rh) => {
                map.insert(rh.version.as_str(), index);
            },
        }
    }
    (map, unreleased_heading)
}

