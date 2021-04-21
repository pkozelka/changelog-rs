use changelog::api::{ChangeSet, ReleaseHeader};
use changelog::{ChangeLog, ChgError};
use std::path::PathBuf;

/// Synchronize new commits into existing `CHANGELOG.md` file.
/// Internally, the process is creating two instances of [`Vec<ChangeSet>`] and carefully adding stuff from one to the other
pub fn cmd_sync(changelog_file: &PathBuf, dir: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let changelog = ChangeLog::import_markdown(&text)?;
    let stop_version = match changelog.releases.get(0) {
        None => None,
        Some((rvs, _)) => Some(rvs.version.clone()),
    };
    let commits = ChangeLog::import_git_commits(dir, stop_version, &changelog.config);
    compare(&changelog, &commits)?; //TODO
    Ok(())
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
fn compare(old: &ChangeLog, new: &ChangeLog) -> Result<(), ChgError> {
    let (old_rvs, _old_changeset) = old
        .releases
        .get(0)
        .expect("TODO: Old changeset has no release yet"); // TODO
                                                           // find all new changesets
    let newcs: Vec<&(ReleaseHeader, ChangeSet)> = new
        .releases
        .iter()
        .take_while(|(rvs, _)| rvs.version != old_rvs.version)
        .collect();

    println!("OLD LAST VERSION: {}", old_rvs.version);
    println!("NEWCS:");
    for (rvs, _) in newcs {
        println!("* {:?}", rvs)
    }
    Ok(())
}
