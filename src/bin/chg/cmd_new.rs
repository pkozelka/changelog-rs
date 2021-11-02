use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use changelog::builder::ChangeLogBuilder;
use changelog::ChangeLogConfig;
use changelog::changeset::ChangesetHeader::Unreleased;

pub fn cmd_new(changelog_file: &PathBuf) -> std::io::Result<()> {
    if changelog_file.exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Changelog already exists in {}", changelog_file.display()),
        ))?;
    }
    let config = ChangeLogConfig::default();
    let mut builder = ChangeLogBuilder::new(config);
    builder.section(Unreleased);
    let changelog = builder.build();
    let mut file = File::create(changelog_file)?;
    changelog.print_markdown(&mut file)?;
    // changelog.to_markdown(&mut std::io::stdout())
    file.flush()?;
    Ok(())
}
