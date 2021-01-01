//! Attempt to use GITHUB api with hubcaps library.
//!
//! Problem: fails
//! ```
//! thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: missing field `merged` at line 1 column 14069
//!
//! Caused by:
//!     missing field `merged` at line 1 column 14069', examples/demo-hubcaps-show-pulls:8:17
//! ```
use futures::prelude::*;
use hubcaps::{Github, Credentials};
use hubcaps::pulls::PullListOptionsBuilder;
use hubcaps::issues::State;

#[tokio::main]
async fn main() {
    run().await.unwrap()
}

async fn run() -> anyhow::Result<()>{

    let personal_token = std::fs::read_to_string("../.config/github_token.txt").unwrap();
    let github = Github::new(
        "my-cool-user-agent/0.1.0",
        Credentials::Token(personal_token),
    )?;

    let repo = github.repo("h2oai", "mojo2");

    let options = PullListOptionsBuilder::default()
        .sort(hubcaps::issues::Sort::Updated)
        .state(State::All)
        .build();

    repo.pulls().iter(&options).try_for_each(|pull| async move {
        println!("{:#?}", pull);
        Ok(())
    }).await?;

    Ok(())

}

