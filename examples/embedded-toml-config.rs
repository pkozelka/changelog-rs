#[macro_use]
extern crate serde_derive;

const
    CONFIG: &str = r##"
[git]
version_pattern = "v*"
[keys]
issue_link = "https://github.com/OWNER/REPO/issues/NUMBER"
issue_key = "#NUMBER"
pr_link = "https://github.com/OWNER/REPO/pulls/NUMBER"
pr_key = "PR#NUMBER"
#commit_link = "https://github.com/OWNER/REPO/commits/HASH"
#commit_
"##;

#[derive(Serialize,Deserialize,Debug)]
struct ChangeLogConfig {
    git: GitConfig,
    keys: KeysConfig,
}
#[derive(Serialize,Deserialize,Debug)]
struct GitConfig {
    version_pattern: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct KeysConfig {
    issue_link: String,
    issue_key: String,
    pr_link: String,
    pr_key: String,
}

fn main() {
    let config: ChangeLogConfig = toml::from_str(CONFIG).unwrap();
    println!("{:?}", config);
    let s = toml::to_string(&config).unwrap();
    println!("{}", s);
}
