#[macro_use]
extern crate log;
extern crate structopt;

use crate::cli::Command;
use anyhow::Result;
use changelog::api::{ChangeLog, VersionSpec};
use changelog::builder::ChangeLogBuilder;
use changelog::imports::from_changelog;
use changelog::imports::from_git_repo::list_tags;
use git2::Repository;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    if let Err(e) = run_cli() {
        error!("ERROR: {:?}", e);
        std::process::exit(1);
    }
}

fn run_cli() -> Result<()> {
    // parse commandline
    let args = cli::Cli::with_logging(module_path!());
    // process options
    // process commands
    match args.cmd {
        Command::NewChangelog { .. } => {
            let mut builder = ChangeLogBuilder::new();
            builder.section(VersionSpec::unreleased());
            let changelog = builder.build();
            print_changelog(&changelog, &mut std::io::stdout())?;
            Ok(())
        }
        Command::InitFromGit {} => {
            let repo = Repository::open(".").unwrap();
            let tags = list_tags(&repo).unwrap();
            let mut builder = ChangeLogBuilder::new();
            builder.section(VersionSpec::unreleased());
            builder.traverse_commits(&repo, &tags).unwrap();
            let changelog = builder.build();
            print_changelog(&changelog, &mut std::io::stdout())?;
            Ok(())
        }
        Command::Info { } => {
            let f = File::open(args.changelog_file)?;
            let changelog = from_changelog::parse(&mut BufReader::new(f))?;
            for section in changelog.versions {
                match section.version_spec {
                    VersionSpec::Unreleased { .. } => {
                        println!("Unreleased");
                    }
                    VersionSpec::Release {
                        version,
                        tag: _,
                        timestamp,
                        yanked,
                    } => {
                        println!(
                            "{} version {}{}: {} items",
                            timestamp.naive_utc().date(),
                            version,
                            if yanked { "(YANKED!)" } else { "" },
                            section.items.len()
                        );
                    }
                }
                for item in section.items {
                    println!("* Refs:{:?}, '{}'", item.refs, item.text)
                }
            }
            Ok(())
        }
        Command::SyncFromGit { .. } => {
            todo!()
        }
    }
}

fn print_changelog(changelog: &ChangeLog, out: &mut dyn Write) -> std::io::Result<()> {
    for release in &changelog.versions {
        match &release.version_spec {
            VersionSpec::Unreleased { .. } => {
                writeln!(out, "## Unreleased")?;
            }
            VersionSpec::Release {
                version,
                tag: _,
                timestamp,
                yanked,
            } => {
                let ts = timestamp.to_string();
                writeln!(
                    out,
                    "## {} - {}{}",
                    version,
                    &ts[0..10],
                    if *yanked { " [YANKED]" } else { "" }
                )?;
            }
        }
        if !release.items.is_empty() {
            writeln!(out)?;
            for item in &release.items {
                write!(out, "- ")?;
                if !item.refs.is_empty() {
                    write!(out, "{}: ", item.refs.join(", "))?;
                }
                writeln!(out, "{} / {}", item.text, item.authors.join(", "))?;
            }
            writeln!(out)?;
        }
    }
    Ok(())
}

mod cli {
    use std::path::PathBuf;
    use structopt::StructOpt;

    /// Changelog toolkit
    #[derive(StructOpt, Debug)]
    #[structopt(name = "chg", global_settings = & [structopt::clap::AppSettings::ColoredHelp])]
    pub struct Cli {
        #[structopt(subcommand)] // Note that we mark a field as a subcommand
        pub cmd: Command,
        /// Logging in verbose mode (-v = DEBUG, -vv = TRACE)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: i8,
        /// Logging in silent mode (-s = WARN, -ss = ERROR, -sss = OFF)
        #[structopt(short, long, parse(from_occurrences))]
        silent: i8,
        #[structopt(short = "f", long = "file", default_value = "CHANGELOG.md")]
        pub changelog_file: PathBuf,
    }

    #[derive(StructOpt, Debug)]
    pub enum Command {
        #[structopt(name = "new")]
        NewChangelog {},
        #[structopt(name = "init")]
        /// Read from git repo
        InitFromGit {},
        /// Show some info about current changelog
        Info {},
        #[structopt(name = "sync")]
        SyncFromGit {},
    }

    impl Cli {
        pub fn with_logging(module: &str) -> Self {
            let cli = Self::from_args();
            // initialize logging
            let level = 2 + cli.verbose - cli.silent;
            stderrlog::new()
                .modules(vec![module, "changelog", module_path!()])
                .quiet(level < 0)
                .verbosity(level as usize)
                .timestamp(stderrlog::Timestamp::Millisecond)
                .init()
                .unwrap();
            /*
                        trace!("trace");
                        debug!("debug");
                        info!("info");
                        warn!("warn");
                        error!("error s={} v={} x={}", cli.silent, cli.verbose, level);
            */
            cli
        }
    }
}
