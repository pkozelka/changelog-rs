use std::path::PathBuf;
use git2::Repository;
use changelog::imports::from_git_repo::list_tags;
use changelog::builder::ChangeLogBuilder;
use changelog::api::VersionSpec;
use std::fs::File;
use std::io::Write;

pub fn cmd_import_git(changelog_file: &PathBuf, dir: &PathBuf, stop_version: Option<String>) -> std::io::Result<()> {
    let repo = Repository::open(dir).unwrap();
    let tags = list_tags(&repo).unwrap();
    let mut builder = ChangeLogBuilder::new();
    builder.section(VersionSpec::unreleased());
    let stop_version = stop_version.as_ref().map(String::as_str);
    builder.traverse_commits(&repo, &tags, stop_version).unwrap();
    let changelog = builder.build();
    let mut file = File::create(changelog_file)?;
    changelog.to_markdown(&mut file)?;
    // changelog.to_markdown(&mut std::io::stdout())
    file.flush()?;
    Ok(())
}
