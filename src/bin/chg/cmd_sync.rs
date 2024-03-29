use std::path::PathBuf;

use changelog::{ChangeLog, ChgError};

/// Synchronize new commits into existing `CHANGELOG.md` file.
/// Internally, the process is creating two instances of [`Vec<ChangeSet>`] and carefully adding stuff from one to the other
pub fn cmd_sync(changelog_file: &PathBuf, dir: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let mut changelog = ChangeLog::import_markdown(&text)?;
    let stop_version = match changelog.releases.get(0) {
        None => None,
        Some((rvs, _)) => Some(rvs.version.clone()),
    };
    let commits = ChangeLog::import_git_commits(dir, stop_version, &changelog.config);
    changelog.sync_from(&commits)?; //TODO
    Ok(())
}
