#[macro_use]
extern crate log;
extern crate structopt;

use std::io::Result;
use crate::cli::Command;
use changelog::builder::ChangeLogBuilder;
use changelog::api::{VersionSpec, ChangeLog};
use git2::Repository;
use changelog::imports::from_git_repo::list_tags;

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
        Command::FromGitRepo { } => {
            let repo = Repository::open(".").unwrap();
            let tags = list_tags(&repo).unwrap();
            let mut builder = ChangeLogBuilder::new();
            builder.section(VersionSpec::unreleased());
            builder.traverse_commits(&repo, &tags).unwrap();
            let changelog = builder.build();
            print_changelog(&changelog);
            Ok(())
        }
    }
}

fn print_changelog(changelog: &ChangeLog) {
    for release in &changelog.versions {
        match &release.version_spec {
            VersionSpec::Unreleased { .. } => {
                println!("## Unreleased");
            }
            VersionSpec::Release { version, tag:_, timestamp, yanked } => {
                let ts = timestamp.to_string();
                println!("## {} - {}{}", version, &ts[0..10], if *yanked { " [YANKED]" } else { "" } );
            }
        }
        if !release.items.is_empty() {
            println!();
            for item in &release.items {
                print!("- ");
                if !item.refs.is_empty() {
                    print!("{}: ", item.refs.join(", "));
                }
                println!("{} / {}", item.text, item.authors.join(", "))
            }
            println!();
        }
    }
}

mod cli {
    use structopt::StructOpt;

    /// Changelog toolkit
    #[derive(StructOpt, Debug)]
    #[structopt(name = "chg", global_settings = & [structopt::clap::AppSettings::ColoredHelp])]
    pub struct Cli {
        #[structopt(subcommand)]  // Note that we mark a field as a subcommand
        pub cmd: Command,
        /// Logging in verbose mode (-v = DEBUG, -vv = TRACE)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: i8,
        /// Logging in silent mode (-s = WARN, -ss = ERROR, -sss = OFF)
        #[structopt(short, long, parse(from_occurrences))]
        silent: i8,
    }

    #[derive(StructOpt, Debug)]
    pub enum Command {
        #[structopt(name = "git")]
        /// Read from git repo
        FromGitRepo {},
    }

    impl Cli {
        pub fn with_logging(module: &str) -> Self {
            let cli = Self::from_args();
            // initialize logging
            let level = 2 + cli.verbose - cli.silent;
            stderrlog::new()
                .modules(vec![
                    module,
                    "changelog",
                    module_path!(),
                ])
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
