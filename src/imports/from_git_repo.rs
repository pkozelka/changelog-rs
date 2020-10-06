use git2::{Repository, Error, Oid};
use chrono::{FixedOffset, TimeZone, DateTime};
use std::collections::HashMap;
use crate::builder::ChangeLogBuilder;
use crate::api::{VersionSpec, ChangeItem, ChangeType};
use crate::imports::commit_msg::{CommitMessageAnalyzer, CommitMessage};

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

fn git_time_to_chrono(time: git2::Time) -> DateTime<FixedOffset>{
    let offset_seconds = time.offset_minutes() * 60;
    let nts = chrono::NaiveDateTime::from_timestamp(time.seconds() + offset_seconds as i64, 0);
    FixedOffset::east(offset_seconds).from_local_datetime(&nts).unwrap()
}

impl ChangeLogBuilder {
    pub fn traverse_commits(&mut self, repo: &Repository, tags: &HashMap<Oid, String>) -> Result<(), Error> {
        let head = repo.head()?;
        let mut commit = head.peel_to_commit()?;
        let cma = CommitMessageAnalyzer::init().unwrap();
        while !commit.id().is_zero() {
            {
                let author = commit.author();
                let ts = git_time_to_chrono(author.when());
/*                println!("{} {} {} <{}>: {}",
                         ts,
                         &commit.id().to_string()[0..7],
                         author.name().unwrap_or("?"),
                         author.email().unwrap_or("?"),
                         subject);
*/                //TODO create a YANKED release section if the text indicates release commit and a) tag containing word `yanked` exists b) there is no release tag
                let msg = commit.message().unwrap_or("");
                let cm = cma.analyze(msg);
                match tags.get(&commit.id()) {
                    None => {
                        match cm {
                            CommitMessage::Contribution { component, refs, subject, details:_ } => {
                                self.item(ChangeItem {
                                    refs,
                                    change_type: ChangeType::Other, // TODO
                                    component,
                                    text: subject,
                                    authors: vec![author.name().unwrap_or("?").to_string()]
                                }).unwrap(); // TODO
                            }
                            CommitMessage::Release { version } => {
                                warn!("Untagged release detected: {}", version);
                            }
                            CommitMessage::PostRelease { ref_ver } => {
                                debug!("Post-release detected, ignoring commit: {}", ref_ver);
                            }
                            CommitMessage::Revert { orig_msg } => {
                                warn!("Revert detected but not implemented yet: '{}'", orig_msg);
                            }
                        }
                    },
                    Some(tag_name) => {
                        // println!("\\--> Tag: {}", tag_name);
                        self.section(VersionSpec::release(tag_name, ts));
                    },
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
