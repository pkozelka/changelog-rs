use changelog::api::ChangeSet;
use changelog::{ChangeLog, ChgError, VersionSpec};
use std::path::PathBuf;

/// Synchronize new commits into existing `CHANGELOG.md` file.
/// Internally, the process is creating two instances of [`Vec<ChangeSet>`] and carefully adding stuff from one to the other
pub fn cmd_sync(changelog_file: &PathBuf, dir: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let changelog = ChangeLog::import_markdown(&text)?;
    let stop_version = latest_release(&changelog.versions);
    let commits = ChangeLog::import_git_commits(dir, stop_version, &changelog.config);
    compare(&changelog.versions, &commits); //TODO
    Ok(())
}

fn latest_release(versions: &[ChangeSet]) -> Option<String> {
    for change_set in versions {
        if let VersionSpec::Release { version, .. } = &change_set.version_spec {
            return Some(version.clone());
        }
    }
    None
}

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
fn compare(old: &[ChangeSet], new: &[ChangeSet]) -> Vec<ChangeSet> {
    let old_lr = latest_release(&old)
        .expect("TODO: Old changeset has no release yet"); // TODO
    // find all new changesets
    let newcs: Vec<&ChangeSet> = new.iter().take_while(|&x| match &x.version_spec {
        VersionSpec::Unreleased { .. } => true,
        VersionSpec::Release { version, .. } => *version != old_lr
    }).collect();

    println!("OLD_LR: {}", old_lr);
    println!("NEWCS:");
    for x in newcs {
        println!("* {:?}", x.version_spec)
    }
    vec![]
}
