use std::str::Lines;

use regex::Regex;

pub enum CommitMessage {
    /// Regular commit that contributes to code and is equipped with some fields
    /// Format is heavily project-dependent, typically manually composed.
    Contribution {
        /// best component name candidate
        component: String,
        /// references to commits, issues, PRs
        refs: Vec<String>,
        /// first line in message
        subject: String,
        /// remaining lines
        details: String,
    },
    /// The commit recording release action; usually switches version to a next release number
    /// Format is heavily project-dependent, sometimes even manually composed.
    Release {
        /// released version
        version: String,
    },
    /// The commit switching project version back to "development" version (SNAPSHOT etc.)
    /// Format is heavily project-dependent, sometimes even manually composed.
    PostRelease {
        /// the referenced version; can be either the next snapshot, next version or previous version, depending on the tooling design
        ref_ver: String,
    },
    /// Special commit counteracting a recent one.
    /// Format is proposed by GIT as default, but can be altered by user.
    /// Example: `Release 1.2.3` or `[BUILD] Released version 1.2.3`
    Revert {
        orig_msg: String,
    }
}

pub struct CommitMessageAnalyzer {
    /// valid components
    // components: Vec<String>,
    // revision_prefix: String,
    // issue_prefix: String,
    pr_mergecommit_regex: Regex,
    pr_squash_regex: Regex,
    // release_prefix: String,
    // postrelease_prefix: String,
}

const GIT_REVERT_PREFIX: &str = "Revert \"";

impl CommitMessageAnalyzer {

    pub fn init() -> Result<Self,regex::Error> {
        Ok(Self {
            pr_mergecommit_regex: Regex::new(r"Merge pull request #(?P<pr>\d+) from (?P<branch>.*)")?,
            pr_squash_regex: Regex::new(r"^(?P<subject>.*) \(#(?P<pr>\d+)\)$")?,
        })
    }

    pub fn analyze(&self, msg: &str) -> CommitMessage {
        let msg = msg.trim();
        // Empty message
        if msg.is_empty() {
            return CommitMessage::Contribution { component: "".to_string(), refs: vec![], subject: "".to_string(), details: "".to_string() };
        }
        // Reverting a commit
        if msg.starts_with(GIT_REVERT_PREFIX) && msg.ends_with('"') {
            return CommitMessage::Revert { orig_msg: msg[GIT_REVERT_PREFIX.len()..msg.len() - 1].to_string() }
        }
        // Github: merged pull-request
        let mut lines = msg.trim().lines();
        let first_line = lines.next().unwrap_or("");

        if let Some((pr,subject)) = self.detect_pr_merge(first_line, &mut lines) {
            return CommitMessage::Contribution {
                component: "".to_string(),
                refs: vec![format!("PR#{}", pr)],
                subject,
                details: "".to_string()
            }
        }
        // otherwise
        CommitMessage::Contribution {
            component: "N/A".to_string(),
            refs: vec![],
            subject: first_line.to_string(),
            details: "".to_string()
        }
    }

    /// Find PR number and "clean" message subject.
    fn detect_pr_merge(&self, first_line: &str, lines: &mut Lines) -> Option<(String, String)> {
        if let Some(captures)  = self.pr_mergecommit_regex.captures(first_line) {
            if let Some(m) = captures.name("pr") {
                let pr = m.as_str().to_string();
                // use first non-empty line
                for line in lines {
                    if !line.trim().is_empty() {
                        return Some((pr, line.to_string()));
                    }
                }
                // ... or just the first line
                return Some((pr, first_line.to_string()))
            }
        }
        if let Some(captures) = self.pr_squash_regex.captures(first_line) {
            if let Some(m) = captures.name("pr") {
                let pr = m.as_str().to_string();
                let subject = match captures.name("subject") {
                    Some(m) => m.as_str(),
                    None => first_line,
                };
                return Some((pr, subject.to_string()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::imports::commit_msg::{CommitMessage, CommitMessageAnalyzer};

    #[test]
    fn revert_commit() {
        let cmp = CommitMessageAnalyzer::init().unwrap();
        let commit = cmp.analyze("Revert \"Some ultracool stuff\"");
        match commit {
            CommitMessage::Revert { orig_msg } => {
                println!("revert_commit: Original message: '{}'", orig_msg);

                assert_eq!(orig_msg, "Some ultracool stuff");
            }
            _  => panic!("")
        }
    }

    #[test]
    fn pr_merge_commit() {
        let cmp = CommitMessageAnalyzer::init().unwrap();
        let commit = cmp.analyze("Merge pull request #1234 from pk/some-pr-branch\n\nHere is the PR title");
        match commit {
            CommitMessage::Contribution { component, refs, subject, details:_ } => {
                println!("pr_merge_commit: {} [{}] {}", refs.join(", "), component, subject);

                assert_eq!(refs[0], "PR#1234");
                assert_eq!(subject, "Here is the PR title");
            }
            _  => panic!("")
        }
    }

    #[test]
    fn pr_merge_squash() {
        let cmp = CommitMessageAnalyzer::init().unwrap();
        let commit = cmp.analyze("[cpp] disable tree shap computing when tree model doesn't use input features (#1073)");
        match commit {
            CommitMessage::Contribution { component, refs, subject, details:_ } => {
                println!("pr_merge_squash: {} [{}] {}", refs.join(", "), component, subject);

                assert_eq!(refs[0], "PR#1073");
                assert_eq!(subject, "[cpp] disable tree shap computing when tree model doesn't use input features");
            }
            _  => panic!("")
        }
    }
}
