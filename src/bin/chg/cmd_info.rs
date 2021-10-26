use std::path::PathBuf;

use changelog::{ChangeLog, ChgError};
use changelog::changeset::ChangeSet;

pub fn cmd_info(changelog_file: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let changelog = ChangeLog::import_markdown(&text)?;
    if let Some(unreleased) = changelog.unreleased {
        println!("Unreleased");
        print_changeset(&unreleased);
    }
    for (rvs, changeset) in &changelog.changesets {
        println!(
            "{} version {}{}: {} items",
            rvs.timestamp,
            rvs.version,
            if rvs.yanked { "(YANKED!)" } else { "" },
            changeset.items.len()
        );
        print_changeset(&changeset)
    }
    Ok(())
}

fn print_changeset(changeset: &ChangeSet) {
    for item in &changeset.items {
        println!("* Refs:{:?}, '{}'", item.refs, item.text)
    }
}
