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
