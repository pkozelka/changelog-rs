use crate::api::{ChangeItem, ChangeType, VersionSpec};
use crate::builder::ChangeLogBuilder;
use crate::imports::commit_msg::{CommitMessage, CommitMessageAnalyzer};
use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Error, Oid, Repository};
use std::collections::HashMap;

pub fn list_tags(repo: &Repository) -> Result<HashMap<Oid, String>, Error> {
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

fn git_time_to_chrono(time: git2::Time) -> DateTime<FixedOffset> {
    let offset_seconds = time.offset_minutes() * 60;
    let nts = chrono::NaiveDateTime::from_timestamp(time.seconds() + offset_seconds as i64, 0);
    FixedOffset::east(offset_seconds)
        .from_local_datetime(&nts)
        .unwrap()
}

impl ChangeLogBuilder {
    pub fn traverse_commits(
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
                let msg = commit.message().unwrap_or("");
                let cm = cma.analyze(msg);
                match tags.get(&commit.id()) {
                    None => {
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
                                    authors: vec![author.name().unwrap_or("?").to_string()],
                                })
                                .unwrap(); // TODO
                            }
                            CommitMessage::Release { version } => {
                                warn!("Untagged release detected: {}", version);
                                self.section(VersionSpec::release(version.as_str(), ts, true));
                                if let Some(stop_version) = stop_version {
                                    if stop_version == version {
                                        trace!("Stopping on version '{}' as requested", version);
                                        break;
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
                    }
                    Some(tag_name) => {
                        let yanked = tag_name.to_uppercase().contains("YANKED"); // TODO: opinion
                        self.section(VersionSpec::release(tag_name, ts, yanked));
                    }
                }
            }
            commit = match commit.parent(0) {
                Ok(c) => c,
                Err(_) => break,
            }
        }
        Ok(())
    }
}
