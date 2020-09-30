use std::fs::File;
use std::io::{Result, BufReader, BufRead, Error, ErrorKind};

fn main() {
    run("data/mojo2.reference.log.txt").unwrap();
}

fn run(changelog: &str) -> std::io::Result<()> {
    let f = File::open(changelog)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{:?}", GitLogReferenceLine::parse(&line?)?);
    }
    Ok(())
}

/// Git log entry produced by `git log --format=reference  --date=iso8601 --abbrev`
#[derive(Debug)]
struct GitLogReferenceLine {
    hash: String,
    /// first line of commit message
    subject: String,
    timestamp: String,
}

impl GitLogReferenceLine {

    fn parse(line: &str) -> Result<Self> {
        let inner_start = line.find(" (")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find hash separator: {}", line)))?;
        if !line.ends_with(")") {
            return Err(Error::new(ErrorKind::InvalidData, format!("Cannot find right brace: {}", line)))
        }

        let inner = &line[inner_start +2..line.len() - 2];
        let text_end = inner.rfind(", 2")
            .ok_or(Error::new(ErrorKind::InvalidData, format!("Cannot find timestamp separator: {}", line)))?;

        let hash = line[0..inner_start].to_string();
        let message = inner[2..text_end].to_string();
        let timestamp = inner[text_end+2..].to_string();
        Ok(Self { hash, subject: message, timestamp })
    }
}