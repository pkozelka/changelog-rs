use changelog::api::ChangeSet;
use changelog::{ChangeLog, ChgError, VersionSpec};
use std::path::PathBuf;

/// Synchronize new commits into existing `CHANGELOG.md` file.
/// Internally, the process is creating two instances of [`Vec<ChangeSet>`] and carefully adding stuff from one to the other
pub fn cmd_sync(changelog_file: &PathBuf, dir: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let changelog = ChangeLog::import_markdown(&text)?;
    let stop_version = latest_release(changelog.versions);
    let commits = ChangeLog::import_git_commits(dir, stop_version, &changelog.config);
    todo!()
}

fn latest_release(versions: Vec<ChangeSet>) -> Option<String> {
    for change_set in &versions {
        if let VersionSpec::Release { version, .. } = &change_set.version_spec {
            return Some(version.clone());
        }
    }
    None
}
