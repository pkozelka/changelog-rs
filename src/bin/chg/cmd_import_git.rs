use changelog::{ChangeLog, ChangeLogConfig};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn cmd_import_git(
    changelog_file: &PathBuf,
    dir: &PathBuf,
    stop_version: Option<String>,
) -> std::io::Result<()> {
    let config = ChangeLogConfig::default();
    let changelog = ChangeLog::import_git_commits(dir, stop_version, &config);
    let mut file = File::create(changelog_file)?;
    changelog.print_markdown(&mut file)?;
    // changelog.to_markdown(&mut std::io::stdout())
    file.flush()?;
    Ok(())
}
