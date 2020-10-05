use git2::{Repository, Error, Oid};
use chrono::{FixedOffset, TimeZone};
use std::collections::HashMap;
use changelog::builder::ChangeLogBuilder;
use changelog::api::{VersionSpec, ChangeLog};

fn main() {
    // let repo = match Repository::open("/home/pk/github.com/contentcheck-maven-plugin") {
    let repo = match Repository::open("/home/pk/h2o/mojo2") {
        // let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let tags = list_tags(&repo).unwrap();
    let changelog = commits(&repo, &tags).unwrap();
    for release in changelog.versions {
        match release.version_spec {
            VersionSpec::Unreleased { .. } => {
                println!("## Unreleased");
            }
            VersionSpec::Release { version, tag:_, date } => {
                println!("## {} - {}", version, &date[0..10]);
            }
        }
    }
}

fn list_tags(repo: &Repository) -> Result<HashMap<Oid, String>, Error> {
    println!("Tags:");
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

fn commits(repo: &Repository, tags: &HashMap<Oid, String>) -> Result<ChangeLog, Error>{

    let mut builder = ChangeLogBuilder::new();
    builder.section(VersionSpec::Unreleased { major: None, branch: None });
    let head = repo.head()?;
    let mut commit = head.peel_to_commit()?;
    loop {
        if commit.id().is_zero() { break; }
        {
            let mut msg = commit.message().unwrap_or("?").trim().lines();
            let subject = msg.next().unwrap_or("-");
            let mut linecnt = 0;
            while msg.next().is_some() { linecnt += 1; }
            let subject_trail = if linecnt == 0 {
                "".to_string()
            } else {
                format!(" ... ({} more lines)", linecnt)
            };
            let author = commit.author();
            // let committer = commit.committer();
            let author_time = author.when();
            let offset_seconds = author_time.offset_minutes() * 60;
            let nts = chrono::NaiveDateTime::from_timestamp(author_time.seconds() + offset_seconds as i64, 0);
            let ts = FixedOffset::east(offset_seconds).from_local_datetime(&nts).unwrap();
            println!("{} {} {} <{}>: {}{}",
                     ts,
                     &commit.id().to_string()[0..7],
                     author.name().unwrap_or("?"),
                     author.email().unwrap_or("?"),
                     subject, subject_trail);
            //TODO create a YANKED release section if the text indicates release commit and a) tag containing word `yanked` exists b) there is no release tag
            match tags.get(&commit.id()) {
                None => {}
                Some(tag_name) => {
                    println!("\\--> Tag: {}", tag_name);
                    let mut version = &tag_name[..];
                    for c in version.chars() {
                        if c.is_ascii_digit() { break };
                        version = &version[1..];
                    }
                    builder.section(VersionSpec::Release {
                        version: version.to_string(),
                        tag: tag_name.to_string(),
                        date: ts.to_string(),
                    })
                }
            }
        }
        commit = match commit.parent(0) {
            Ok(c) => c,
            Err(_) => break,
        }
    }
    Ok(builder.build())
}

fn _reflog(repo: &Repository) -> Result<(), git2::Error>{
    let reflog = repo.reflog("HEAD")?;
    println!("REFLOG:");
    for entry in reflog.iter() {
        let c = entry.committer();
        let _parent_hash = entry.id_old();
        let hash = entry.id_new();
        let msg = entry.message().unwrap_or("?");
        println!("{} {} <{}> MSG: {}", &hash.to_string()[0..7], c.name().unwrap_or("?"), c.email().unwrap_or("?"), msg);
    }
    Ok(())
}
