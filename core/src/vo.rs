use entity::projects;
use sea_orm::entity::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectVo {
    pub id: i32,
    pub name: String,
    pub repo: String,
    pub repo_id: i32,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub user_id: i32,
    pub username: String,
}

impl ProjectVo {
    pub fn new(project: projects::Model, username: String) -> ProjectVo {
        ProjectVo {
            id: project.id,
            name: project.name,
            repo: project.repo,
            repo_id: project.repo_id,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
            user_id: project.user_id,
            username,
        }
    }
}
