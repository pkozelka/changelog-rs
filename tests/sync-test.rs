use changelog::{ChangeLog, ChgError};

#[test]
fn empty() {
    //TODO maybe we should fail here?
    ChangeLog::import_markdown(" ").unwrap();
}

#[test]
fn garbage() {
    //TODO should we require "ChangeLog" title?
    ChangeLog::import_markdown("#ABCD").unwrap();
}

#[test]
fn invalid_version_id() {
    let result = ChangeLog::import_markdown(r###"# Changelog
## blahblah whatever nonsense
"###);
    match result {
        Err(ChgError::InvalidVersionID(v, rh)) => {
            assert_eq!(v, "blahblah");
            assert_eq!(rh, "blahblah whatever nonsense");
        }
        Err(e) => panic!("Different error expected - got {:?}", e),
        Ok(_) => panic!("Error expected"),
    }
}

#[test]
fn invalid_timestamp() {
    let result = ChangeLog::import_markdown(r###"# Changelog
## 1.2.3-alpha-1 whatever nonsense
"###);
    match result {
        Err(ChgError::InvalidTimestamp(ts,rh)) => {
            assert_eq!(ts, "whatever");
            assert_eq!(rh, "1.2.3-alpha-1 whatever nonsense");
        }
        Err(e) => panic!("Different error expected - got {:?}", e),
        Ok(_) => panic!("Error expected"),
    }
}

#[test]
fn header_garbage() {
    let changelog = ChangeLog::import_markdown(r###"# Changelog
## 1.2.3-alpha-1 1972-05-31 noise
"###).unwrap();
    assert!(changelog.unreleased.is_none(), "No unreleased sections expected");
    assert_eq!(1, changelog.releases.len());
    assert_eq!("\n# Changelog", changelog.prolog, "prolog");
}

