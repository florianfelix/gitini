#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use clap::{Command, Arg};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

mod api_calls;
mod config_settings;
mod newrepo;
mod runtime_settings;
mod utils;
use api_calls::{create_repo, getgit};
use config_settings::ConfigSettings;
use runtime_settings::RuntimeSettings;

#[tokio::main]
async fn execute(
    config: ConfigSettings,
    settings: RuntimeSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    // Auth Token
    let auth = format!("token {}", &config.api_key).to_string();

    // Headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("gitini"));

    // Client
    let mut client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // DoIt
    create_repo(&mut client, settings).await?;
    Ok(())
}

fn main() {
    // Settings for this run
    let working_dir = std::env::current_dir().ok();
    let repo_name = working_dir
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(String::from)
        .unwrap(); // Is this truly the best way?

    let mut settings = RuntimeSettings::new(working_dir.unwrap(), repo_name, true);

    // Load - Create Config
    let confname = "gitini";
    let mut config: ConfigSettings = confy::load(confname).unwrap();

    // ARGUMENTS
    let matches = Command::new("gitini")
        .version("0.4")
        .author("Florian Felix M. <florianfelixmeyer@gmail.com>")
        .about("gitini this Folder")
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .value_name("TOKEN")
                .help("Store the Github API Token")
                .takes_value(true),
        )
        .arg(
            Arg::new("public")
                .short('p')
                .long("public")
                .help("Create a public repository"),
        )
        .arg(
            Arg::new("complete")
                .short('c')
                .long("complete")
                .help("Init, commit and push everything"),
        )
        .arg(
            Arg::new("no_browser")
                .short('n')
                .long("no_browser")
                .help("Do not open the repository in browser"),
        )
        .get_matches();

    // TEST ARGUMENTS
    // Store Token in config
    if let Some(t) = matches.value_of("token") {
        config.api_key = t.to_string();
        confy::store(confname, &config).unwrap();
        println!(
            "Stored Token {:?}\n You can now use gitini!",
            &config.api_key
        );
        return;
    }

    // Ask for token and store in config if empty
    if config.api_key.is_empty() {
        let s = utils::read_input();
        config.api_key = s;
        confy::store(confname, &config).unwrap();
        println!(
            "Stored Token {:?}\n You can now use gitini!",
            &config.api_key
        );
        return;
    }

    // Settings: set public
    if matches.is_present("public") {
        settings.private = false;
    }
    if matches.is_present("complete") {
        settings.complete = true;
    }
    if matches.is_present("no_browser") {
        settings.open_website = false;
    }

    // DoIt
    execute(config, settings).unwrap();
}
