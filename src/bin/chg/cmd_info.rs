use std::path::PathBuf;

use changelog::{ChangeLog, ChgError};
use changelog::changeset::ChangeSet;

pub fn cmd_info(changelog_file: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let changelog = ChangeLog::import_markdown(&text)?;
    for changeset in &changelog.changesets {
        print_changeset(&changeset)
    }
    Ok(())
}

fn print_changeset(changeset: &ChangeSet) {
    for item in &changeset.items {
        println!(
            "{:?}: {} items",
            changeset.header,
            changeset.items.len()
        );
        println!("* Refs:{:?}, '{}'", item.refs, item.text)
    }
}
