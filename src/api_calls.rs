use crate::create_settings::CreateSettings;
use crate::newrepo::{CreatedRepo, NewRepoComplete};
use reqwest::Client;

use std::process::Command;

pub async fn getgit(client: &mut Client, url: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("getting: {:#?}", url);
    let res = client.get(url).send().await?;
    println!("\n\nResponse:\n{:#?}", res.text().await.unwrap());

    Ok(())
}

pub async fn create_repo(
    client: &mut reqwest::Client,
    settings: CreateSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = settings.working_dir.file_name().unwrap().to_str().unwrap();
    let nr = NewRepoComplete::new(name, settings.private);
    let url = "https://api.github.com/user/repos".to_string();
    let res = client.post(url).json(&nr).send().await?;

    let status = res.status();
    let content: serde_json::Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();

    if status == 422 {
        println!("Failed. Repository already exists.");
        // println!("\n\nResponse:\n{:#?}", &content["errors"][0]);
    }

    if status == 201 {
        println!("SUCCESS. Repository created");
        let ssh_url: String = content["ssh_url"].as_str().unwrap().into();
        let newrepo: CreatedRepo = CreatedRepo::new().set_ssh_url(ssh_url);
        // println!("\nNEW REPO:\n{:#?}", &newrepo);
        newrepo.push_all();
    }

    // println!("\n\nResponse:\n{:#?}", &content);
    Ok(())
}
