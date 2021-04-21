use changelog::builder::ChangeLogBuilder;
use changelog::ChgError;
use changelog::{ChangeLogConfig, VersionSpec};
use std::path::PathBuf;

pub fn cmd_info(changelog_file: &PathBuf) -> Result<(), ChgError> {
    let text = std::fs::read_to_string(changelog_file)?;
    let config = ChangeLogConfig::default();
    let mut builder = ChangeLogBuilder::new(config);
    builder.parse(&text)?;
    let changelog = builder.build();
    for section in changelog.versions {
        match section.version_spec {
            VersionSpec::Unreleased { .. } => {
                println!("Unreleased");
            }
            VersionSpec::Release {
                version,
                tag: _,
                timestamp,
                yanked,
            } => {
                println!(
                    "{} version {}{}: {} items",
                    timestamp,
                    version,
                    if yanked { "(YANKED!)" } else { "" },
                    section.items.len()
                );
            }
        }
        for item in section.items {
            println!("* Refs:{:?}, '{}'", item.refs, item.text)
        }
    }
    Ok(())
}
