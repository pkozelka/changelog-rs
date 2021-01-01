//! This is an attemnpt to use GITHUB api with octocrab.
//! Problem: fails with private repos:
//! ```
//! Error: GitHub { source: GitHubError { documentation_url: "https://docs.github.com/rest/reference/issues#list-repository-issues", errors: None, message: "Not Found" }, backtrace: Backtrace(   0: <snafu::backtrace_shim::Backtrace as snafu::GenerateBacktrace>::generate
//!             at /home/pk/.cargo/registry/src/github.com-1ecc6299db9ec823/snafu-0.6.10/src/backtrace_shim.rs:15:19
//!      octocrab::map_github_error::{{closure}}
//!      ...
//! ```

use octocrab::OctocrabBuilder;
use octocrab::params::State;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let personal_token = std::fs::read_to_string("../.config/github_token.txt").unwrap();
    let octocrab = octocrab::initialise(OctocrabBuilder::new().personal_token(personal_token))?;
    // let octocrab = octocrab::instance();

    // let issues = octocrab.issues("h2oai", "mojo2")
    let issues = octocrab.issues("pkozelka", "contentcheck-maven-plugin")
        .list()
        .state(State::All)
        .send().await?;
    for rec in issues {
        let closed_info = match rec.closed_at {
            None => "",
            Some(_) => "(closed)",
        };
        println!("#{}{}: {} / {} -> {:?}", rec.number, closed_info, rec.title, rec.user.login, rec.assignees)
    }

    Ok(())
}
