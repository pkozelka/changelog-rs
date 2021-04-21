#[macro_use]
extern crate log;
extern crate structopt;

use anyhow::Result;

use crate::cli::Command;

mod cmd_import_git;
mod cmd_info;
mod cmd_new;
mod cmd_sync;

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
        Command::NewChangelog {} => cmd_new::cmd_new(&args.changelog_file).map_err(|e| e.into()),
        Command::InitFromGit { stop_version } => {
            cmd_import_git::cmd_import_git(&args.changelog_file, &args.dir, stop_version)
                .map_err(|e| e.into())
        }
        Command::Info {} => cmd_info::cmd_info(&args.changelog_file).map_err(|e| e.into()),
        Command::SyncFromGit { .. } => {
            cmd_sync::cmd_sync(&args.changelog_file, &args.dir).map_err(|e| e.into())
        }
    }
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

        /// Changelog file location and name
        #[structopt(short = "f", long = "file", default_value = "CHANGELOG.md")]
        pub changelog_file: PathBuf,

        /// Project directory
        #[structopt(long = "dir", default_value = ".")]
        pub dir: PathBuf,
    }

    #[derive(StructOpt, Debug)]
    pub enum Command {
        #[structopt(name = "new")]
        NewChangelog {},
        #[structopt(name = "init")]
        /// Read from git repo
        InitFromGit {
            /// stop parsing git on some version (must match exactly!)
            #[structopt(long = "stop-version")]
            stop_version: Option<String>,
        },
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
            cli
        }
    }
}
