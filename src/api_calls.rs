use crate::create_settings::CreateSettings;
use crate::newrepo::NewRepoComplete;
use reqwest::Client;

use std::process::Command;

pub async fn getgit(client: &mut Client, url: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("getting: {:#?}", url);
    let res = client.get(url).send().await?;
    println!("\n\nResponse:\n{:#?}", res.text().await.unwrap());

    Ok(())
}

#[derive(Debug)]
struct CreatedRepo {
    ssh_url: String,
}

impl CreatedRepo {
    fn new(ssh_url: String) -> Self {
        Self { ssh_url }
    }
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
        println!("\n\nResponse:\n{:#?}", &content["errors"][0]);
    }

    if status == 201 {
        println!("SUCCESS. Repository created");
        let ssh_url = &content["ssh_url"];
        let newrepo = CreatedRepo::new(ssh_url.as_str().unwrap().into());
        println!("\nNEW REPO:\n{:#?}", &newrepo);
        Command::new("git")
            .arg("init")
            .output()
            .expect("failed to init git");
        Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .expect("failed to stage all files");
        Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("init")
                .output()
                .expect("failed to commit staged files");
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&newrepo.ssh_url)
            .output()
            .expect("failed to set remote");
        Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg("main")
            .output()
            .expect("failed to push to github");
    }

    // println!("\n\nResponse:\n{:#?}", &content);
    Ok(())
}
