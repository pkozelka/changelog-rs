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
                    todo!()
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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{ChangeItem, ChangeSet, ChangeType, ReleaseHeader};
    use crate::changeset::ChangesetHeader;
    use crate::changeset::ChangesetHeader::*;
    use crate::sync::sync_one_from;

    fn init_logging() {
        stderrlog::new()
            .modules(vec!["changelog", module_path!()])
            .verbosity(5)
            .timestamp(stderrlog::Timestamp::Millisecond)
            .init()
            .unwrap();
        log::debug!("logging {}", module_path!());
    }

    fn someday() -> NaiveDate {
        NaiveDate::from_ymd(2021, 11, 2)
    }

    fn tst_item(text: &str) -> ChangeItem {
        ChangeItem {
            refs: vec![],
            change_type: ChangeType::Other,
            component: "".to_string(),
            text: text.to_string(),
            authors: vec![],
        }
    }

    fn rlsh(version: &str) -> ChangesetHeader {
        Release(ReleaseHeader {
            version: version.to_string(),
            tag: "".to_string(),
            timestamp: someday(),
            yanked: false,
        })
    }

    fn print(title: &str, changesets: &[ChangeSet]) {
        println!("--- {} ---", title);
        for changeset in changesets {
            match &changeset.header {
                Unreleased => println!("Unreleased"),
                Release(rh) => println!("Release {}", rh.version)
            }
            for item in &changeset.items {
                println!("* {}", item.text);
            }
        }
    }

    #[test]
    fn test_sync_steps() -> anyhow::Result<()> {
        init_logging();
        trace!("hi");
        let c1 = vec![
            ChangeSet { header: Unreleased, items: vec![tst_item("u1, unreleased")] },
            ChangeSet { header: rlsh("1.0.7"), items: vec![tst_item("r107, main change")] },
            ChangeSet { header: rlsh("1.0.6"), items: vec![tst_item("r106, sixth change")] },
            ChangeSet { header: rlsh("1.0.5"), items: vec![tst_item("r105, fifth change")] },
            ChangeSet { header: rlsh("1.0.4"), items: vec![tst_item("r104, fourth change")] },
            ChangeSet { header: rlsh("1.0.3"), items: vec![tst_item("r103, third change")] },
            ChangeSet { header: rlsh("1.0.2"), items: vec![tst_item("r102, second change")] },
            ChangeSet { header: rlsh("1.0.1"), items: vec![tst_item("r101, first change")] },
            ChangeSet { header: rlsh("1.0.0"), items: vec![tst_item("r100, initial change")] },
        ];

        let mut changelog = Vec::new();
        assert_eq!(changelog.len(), 0, "new changelog must be empty");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 1, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 2, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        print("v3", &changelog);
        assert_eq!(changelog.len(), 3, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 4, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 5, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 6, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        assert_eq!(changelog.len(), 7, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        print("v8", &changelog);
        assert_eq!(changelog.len(), 8, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        print("v9 (1)", &changelog);
        assert_eq!(changelog.len(), 9, "new changelog size mismatch");

        assert!(sync_one_from(&mut changelog, &c1)?);
        print("v9 (2)", &changelog);
        assert_eq!(changelog.len(), 9, "new changelog size mismatch");

        Ok(())
    }
}
