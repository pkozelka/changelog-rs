use std::fs::File;
use std::io::{Result, BufReader, BufRead, Error, ErrorKind};

fn main() {
    run("data/mojo2.log.txt").unwrap();
}

fn run(changelog: &str) -> std::io::Result<()> {
    let f = File::open(changelog)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{:?}", GitLogCustomLine::parse(&line?)?);
    }
    Ok(())
}

/// Git log entry produced by `git log --pretty=format:'%aI %d %H %ae %aN: %s' --first-parent`
#[derive(Debug)]
struct GitLogCustomLine {
    timestamp: String,
    hash: String,
    refs: Vec<String>,
    author_email: String,
    author_name: String,
    /// first line of commit message
    subject: String,
    /// pull-request ID, without leading hash and surrounding braces
    pr: Option<String>,
}

/// Consumes string up to a separator.
/// Returns None if no char was consumed, otherwise Some(left, right) where right starts *after* the separator.
fn split_eat<'a> (line: &'a str, separator: &'a str) -> Option<(&'a str, &'a str)> {
    match line.find(separator) {
        None => None,
        Some(n) => {
            Some((&line[0..n], &line[n + separator.len()..]))
        }
    }
}

impl GitLogCustomLine {

    fn parse(line: &str) -> Result<Self> {
        let (timestamp, rest) = split_eat(line, " ")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find timestamp: {}", line)))?;
        let rest = rest.trim_start();
        let (refs, rest) = if rest.starts_with("(") {
            let (inner, rest) = split_eat(&rest[1..], ") ")
                .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot read refs: {}", rest)))?;
            let refs = inner.split_terminator(", ")
                .map(|x| x.to_string())
                .collect();
            (refs, rest)
        } else {
            (Vec::new(), rest)
        };
        let (hash, rest) = split_eat(rest, " ")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find hash: {}", rest)))?;
        let (author_email, rest) = split_eat(rest, " ")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find author email: {}", rest)))?;
        let (author_name, rest) = split_eat(rest, ": ")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find author name: {}", rest)))?;

        let (subject, pr) = match rest.rfind(" (#") {
            None => {
                const REGULAR_MERGE_PREFIX: &str = "Merge pull request #";
                if rest.starts_with(REGULAR_MERGE_PREFIX) {
                    let (pr, _branch) = split_eat(&rest[REGULAR_MERGE_PREFIX.len()..], " from ")
                        .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot parse regular merge message: {}", rest)))?;
                    (rest, Some(pr.to_string()))
                } else {
                    (rest, None)
                }
            },
            Some(n) => if rest.ends_with(")") && rest.len() - n < 9 {
                // pull request ID at the end, after "merge by squashing"
                (&rest[0..n], Some(rest[n+2..rest.len() - 1].to_string()))
            } else {
                (rest, None)
            }
        };
        Ok(Self {
            hash: hash.to_string(),
            refs,
            author_email: author_email.to_string(),
            author_name: author_name.to_string(),
            subject: subject.to_string(),
            timestamp: timestamp.to_string(),
            pr,
        })
    }
}