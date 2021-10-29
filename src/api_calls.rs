use crate::create_settings::CreateSettings;
use crate::newrepo::NewRepoComplete;
use reqwest::Client;

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
    println!("\n\nResponse:\n{:#?}", res.text().await.unwrap());
    Ok(())
}
