#[macro_use]
extern crate serde_derive;

const
    CONFIG: &str = r##"
[git]
version_pattern = "v*"
[keys]
issue_link = "https://github.com/h2oai/mojo2/issues/{number}"
issue_key = "(?P<number>#\\d+)"
pr_link = "https://github.com/h2oai/mojo2/pulls/{number}"
pr_key = "(?P<number>PR#\\d+)"
"##;

// h2oai/h2oai#22327
// #1234

#[derive(Serialize,Deserialize,Debug, Default)]
struct ChangeLogConfig {
    git: GitConfig,
    keys: KeysConfig,
}
#[derive(Serialize,Deserialize,Debug, Default)]
struct GitConfig {
    version_pattern: String,
}

#[derive(Serialize,Deserialize,Debug, Default)]
struct KeysConfig {
    #[serde(default="xyz")]
    issue_link: String,
    issue_key: String,
    pr_link: String,
    pr_key: String,
}

fn xyz() -> String {
    "hello".to_string()
}

fn main() {
    let config: ChangeLogConfig = toml::from_str(CONFIG).unwrap();
    println!("{:?}", config);
    let s = toml::to_string(&config).unwrap();
    println!("{}", s);

    let mut d = ChangeLogConfig::default();
    d.keys.issue_link = "xyz".to_string();
    println!("d: {}", toml::to_string(&d).unwrap())
}
