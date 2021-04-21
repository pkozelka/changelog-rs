use std::path::PathBuf;
use std::fs::File;
use changelog::imports::from_changelog;
use std::io::BufReader;
use changelog::VersionSpec;
use changelog::ChgError;

pub fn cmd_info(changelog_file: &PathBuf) -> Result<(), ChgError> {
    let f = File::open(changelog_file)?;
    let changelog = from_changelog::parse(&mut BufReader::new(f))?;
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
                    timestamp.naive_utc().date(),
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
