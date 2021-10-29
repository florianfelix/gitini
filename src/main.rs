#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use clap::{App, Arg};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

mod api_calls;
mod create_settings;
mod gconfig;
mod newrepo;
mod utils;
use api_calls::{create_repo, getgit};
use create_settings::CreateSettings;
use gconfig::GitifyConfig;

#[tokio::main]
async fn execute(
    config: GitifyConfig,
    settings: CreateSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let auth = format!("token {}", &config.api_key).to_string();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("gitify"));

    let mut client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    create_repo(&mut client, settings).await?;

    // let repos = "https://api.github.com/user/repos".to_string();
    // let base = "https://api.github.com/users/florianfelix/repos".to_string();
    // let zen = "https://api.github.com/zen".to_string();

    // getgit(&mut client, repos).await?;
    // getgit(&mut client, zen).await?;

    Ok(())
}

fn main() {
    let working_dir = std::env::current_dir().unwrap();
    println!("Working dir:\n{:?}", &working_dir);

    let confname = "gitify.conf";
    let mut config: GitifyConfig = confy::load(confname).unwrap();
    let mut settings = CreateSettings::new(working_dir, true);

    let matches = App::new("Gitify")
        .version("0.1")
        .author("Florian Felix M. <florianfelixmeyer@gmail.com>")
        .about("Gitify this Folder")
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .value_name("TOKEN")
                .about("Store the Github API Token")
                .takes_value(true),
        )
        .arg(
            Arg::new("public")
                .short('p')
                .long("public")
                // .value_name("PUBLIC")
                .about("Create a public repository"),
        )
        .get_matches();

    if let Some(t) = matches.value_of("token") {
        config.api_key = t.to_string();
        confy::store(confname, &config).unwrap();
        println!(
            "Stored Token {:?}\n You can now use gitify!",
            &config.api_key
        );
        return;
    }

    if config.api_key.is_empty() {
        let s = utils::read_input();
        config.api_key = s;
        confy::store(confname, &config).unwrap();
        println!(
            "Stored Token {:?}\n You can now use gitify!",
            &config.api_key
        );
        return;
    }

    if matches.is_present("public") {
        settings.private = false;
    }

    println!("{:?}", &config);
    println!("{:?}", &settings);
    execute(config, settings).unwrap();
}
