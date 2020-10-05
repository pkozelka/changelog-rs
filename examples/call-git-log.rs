use git2::{Repository, Error, Oid};
use chrono::{FixedOffset, TimeZone};
use std::collections::HashMap;

fn main() {
    // let repo = match Repository::open("/home/pk/github.com/contentcheck-maven-plugin") {
    let repo = match Repository::open("/home/pk/h2o/mojo2") {
        // let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let tags = list_tags(&repo).unwrap();
    commits(&repo, &tags).unwrap();
}

fn list_tags(repo: &Repository) -> Result<HashMap<Oid, String>, Error> {
    println!("Tags:");
    let mut tags: HashMap<Oid, String> = HashMap::new();
    repo.tag_foreach(|oid, bytes| {
        let tag_name = String::from_utf8_lossy(bytes);
        if tag_name.starts_with("refs/tags/") {
            tags.insert(oid, tag_name[10..].to_string());
        }
        true
    })?;

    for (tag_id, name) in &tags {
        println!("tag: {} = {}", tag_id.to_string(), name)
    }
    Ok(tags)
}

fn commits(repo: &Repository, tags: &HashMap<Oid, String>) -> Result<(), Error>{

    let head = repo.head()?;
    let mut commit = head.peel_to_commit()?;
    return loop {
        if commit.id().is_zero() {
            break Ok(());
        }
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
            // TODO: find tags matching this commit OID
            match tags.get(&commit.id()) {
                None => {}
                Some(tag_name) => {
                    println!("\\--> Tag: {}", tag_name);
                }
            }
        }
        commit = match commit.parent(0) {
            Ok(c) => c,
            Err(_) => break Ok(()),
        }
    }
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
