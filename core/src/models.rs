use sea_orm::entity::prelude::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Deserialize, Default)]
pub struct GroupFilter {
    pub name: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct ConfigFilter {
    pub group_id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct NewGroup {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateGroup {
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewConfig {
    pub group_id: i32,
    pub name: String,
    pub values: Option<Json>,
}

#[derive(Default, Deserialize)]
pub struct UpdateConfig {
    pub group_id: Option<i32>,
    pub name: Option<String>,
    pub values: Option<Json>,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Deserialize)]
pub struct TemplateFilter {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub tag: Option<String>,
}

#[derive(Deserialize)]
pub struct NewTemplate {
    pub name: String,
    pub brief: String,
    pub repo: String,
}

#[derive(Deserialize)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub brief: Option<String>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub repo: Option<String>,
}

#[derive(Deserialize)]
pub struct NewProject {
    pub name: String,
    pub repo: String,
    pub repo_id: i32,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct ProjectFilter {
    pub name: Option<String>,
    pub description: Option<String>,
}
