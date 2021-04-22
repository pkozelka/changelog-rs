use crate::api::{ChangeItem, ChangeType, ReleaseHeader};
use crate::builder::ChangeLogBuilder;
use crate::imports::commit_msg::{CommitMessage, CommitMessageAnalyzer};
use crate::{ChangeLog, ChangeLogConfig};
use chrono::NaiveDate;
use git2::{Error, Oid, Repository};
use std::collections::HashMap;
use std::path::Path;

fn list_tags(repo: &Repository) -> Result<HashMap<Oid, String>, Error> {
    let mut tag_objects: HashMap<Oid, String> = HashMap::new();
    repo.tag_foreach(|oid, bytes| {
        let ref_name = String::from_utf8_lossy(bytes);
        let tag_name = if ref_name.starts_with("refs/tags/") {
            &ref_name[10..]
        } else {
            ref_name.as_ref()
        };
        tag_objects.insert(oid, tag_name.to_string());
        true
    })?;

    let mut tags = HashMap::new();
    for (oid, tag_name) in tag_objects {
        match repo.find_tag(oid) {
            Ok(heavy_tag) => {
                // Heavy tag: oid is stored inside the object
                let oid = heavy_tag.target()?.id();
                tags.insert(oid, tag_name);
            }
            Err(_) => {
                // Lightweight tag: its oid equals target
                tags.insert(oid, tag_name);
            }
        }
    }
    Ok(tags)
}

fn git_time_to_chrono(time: git2::Time) -> NaiveDate {
    let offset_seconds = time.offset_minutes() * 60;
    chrono::NaiveDateTime::from_timestamp(time.seconds() + offset_seconds as i64, 0).date()
}

impl ChangeLogBuilder {
    /// Traverses commit from current HEAD backwards, until a release commit with `stop_version` is encountered, or until the end.
    ///
    /// TODO pre-process tags to
    /// * a) only contain release tags
    /// * b) relate them to main path in case they are on release branch (like 2.5.12 in mojo2)
    /// Any release commits with no related tags are considered yanked. So are commits with special yanked tags.
    fn traverse_commits(
        &mut self,
        repo: &Repository,
        tags: &HashMap<Oid, String>,
        stop_version: Option<&str>,
    ) -> Result<(), Error> {
        let head = repo.head()?;
        let mut commit = head.peel_to_commit()?;
        let cma = CommitMessageAnalyzer::init().unwrap();
        while !commit.id().is_zero() {
            {
                let author = commit.author();
                let ts = git_time_to_chrono(author.when());
                match tags.get(&commit.id()) {
                    None => {
                        let msg = commit.message().unwrap_or("");
                        let cm = cma.analyze(msg);
                        if !self.handle_untagged_commit(stop_version, author.name().unwrap_or("?"), ts, cm) {
                            break;
                        }
                    }
                    Some(tag_name) => {
                        if let Some(version) = self.tag_name_to_version(tag_name) {
                            let yanked = tag_name.to_uppercase().contains("YANKED"); // TODO: configurable
                            self.section(Some(ReleaseHeader::release_tagged(
                                tag_name,
                                version.as_str(),
                                ts,
                                yanked,
                            )));

                            if let Some(stop_version) = stop_version {
                                if stop_version == version {
                                    trace!("Stopping on version '{}' as requested", version);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            if commit.parent_count() == 0 { break }
            commit = commit.parent(0)?;
        }
        Ok(())
    }

    /// Processes commit without a release tag.
    ///
    /// Returns false if processing further commits should stop (based on [`stop_version`])
    fn handle_untagged_commit(&mut self, stop_version: Option<&str>, author: &str, ts: NaiveDate, cm: CommitMessage) -> bool {
        match cm {
            CommitMessage::Contribution {
                component,
                refs,
                subject,
                details: _,
            } => {
                self.item(ChangeItem {
                    refs,
                    change_type: ChangeType::Other, // TODO
                    component,
                    text: subject,
                    authors: vec![author.to_string()],
                })
                    .unwrap(); // TODO
            }
            CommitMessage::Release { version } => {
                if let Some(rh) = ReleaseHeader::release(version.as_str(), ts, true)
                {
                    warn!(
                        "Untagged release detected: {} - will be considered yenked",
                        version
                    );
                    self.section(Some(rh));
                    if let Some(stop_version) = stop_version {
                        if stop_version == version {
                            trace!(
                                "Stopping on version '{}' as requested",
                                version
                            );
                            return false;
                        }
                    }
                }
            }
            CommitMessage::PostRelease { ref_ver } => {
                debug!("Post-release detected, ignoring commit: {}", ref_ver);
            }
            CommitMessage::Revert { orig_msg } => {
                warn!("Revert detected but not implemented yet: '{}'", orig_msg);
            }
        }
        return true;
    }

    fn tag_name_to_version(&self, tag_name: &str) -> Option<String> {
        if tag_name.starts_with("v") {
            //TODO: configurable
            Some(tag_name[1..].to_owned())
        } else {
            None
        }
    }
}

impl ChangeLog {
    pub fn import_git_commits<D: AsRef<Path>>(
        dir: D,
        stop_version: Option<String>,
        config: &ChangeLogConfig,
    ) -> Self {
        let repo = Repository::open(dir).unwrap();
        let tags = list_tags(&repo).unwrap();
        let mut builder = ChangeLogBuilder::new(config.clone());
        builder.section(None);
        let stop_version = stop_version.as_ref().map(String::as_str);
        builder
            .traverse_commits(&repo, &tags, stop_version)
            .unwrap();
        builder.build()
    }
}
