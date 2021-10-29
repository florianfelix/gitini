use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRepoComplete {
    name: String,
    description: String,
    homepage: String,
    private: bool,
    has_issues: bool,
    has_projects: bool,
    has_wiki: bool,
    team_id: i32,
    auto_init: bool,
    gitignore_template: String,
    licence_template: String,
    allow_squash_merge: bool,
    allow_merge_commit: bool,
    allow_rebase_merge: bool,
    allow_auto_merge: bool,
    delete_branch_on_merge: bool,
    has_downloads: bool,
    is_template: bool,
}

impl NewRepoComplete {
    pub fn new(name: &str, private: bool) -> Self {
        Self {
            name: name.to_string(),
            description: "String".to_string(),
            homepage: "String".to_string(),
            private: private,
            has_issues: true,
            has_projects: true,
            has_wiki: true,
            team_id: 0,
            auto_init: false,
            gitignore_template: "".to_string(),
            licence_template: "String".to_string(),
            allow_squash_merge: true,
            allow_merge_commit: true,
            allow_rebase_merge: true,
            allow_auto_merge: false,
            delete_branch_on_merge: false,
            has_downloads: true,
            is_template: false,
        }
    }
}

#[derive(Debug)]
pub struct CreatedRepo {
    pub ssh_url: String,
}

impl CreatedRepo {
    pub fn new() -> Self {
        Self { ssh_url: "".into() }
    }
    pub fn set_ssh_url(mut self, ssh_url: String) -> Self {
        self.ssh_url = ssh_url;
        self
    }
    pub fn push_all(&self) {
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
            .arg(&self.ssh_url)
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
}