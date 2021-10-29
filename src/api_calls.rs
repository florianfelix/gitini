use crate::create_settings::CreateSettings;
use crate::newrepo::{CreatedRepo, NewRepoComplete};
use reqwest::Client;

use std::process::Command;

// unused for now
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
    // Post Body
    let nr = NewRepoComplete::new(&settings.repo_name, settings.private);
    // Repositories Endpoint
    let url = "https://api.github.com/user/repos".to_string();
    // make post request
    let res = client.post(url).json(&nr).send().await?;

    let status = res.status();

    // turn response into json
    let content: serde_json::Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();

    // Repository already exists
    if status == 422 {
        println!("Failed. Repository already exists.");
        return Ok(());
    }

    // TODO make things better in case of failure
    if status != 201 {
        println!("Failed for some unspecified Reason.");
        return Ok(());
    }

    // Create Repo and upload everything
    if status == 201 {
        println!("SUCCESS. Repository created");
        let ssh_url: String = content["ssh_url"].as_str().unwrap().into();
        let newrepo: CreatedRepo = CreatedRepo::new().set_ssh_url(ssh_url);

        newrepo.check_gitignore(settings.working_dir.clone());
        newrepo.check_readme(settings.working_dir.clone());

        if settings.complete {
            newrepo.push_all();
            return Ok(())
        }
        
        newrepo.base_case();
    }

    Ok(())
}
