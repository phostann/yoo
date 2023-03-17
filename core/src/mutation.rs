use ::entity::{
    configs, configs::Entity as Config, groups, groups::Entity as Group, projects,
    projects::Entity as Project, templates, templates::Entity as Template, users,
    users::Entity as User,
};
use chrono::Local;
use sea_orm::{ActiveValue::Set, *};

use crate::{
    NewConfig, NewGroup, NewProject, NewTemplate, ProjectVo, UpdateConfig, UpdateGroup,
    UpdateProject, UpdateTemplate,
};

pub struct Mutation;

impl Mutation {
    // create group
    pub async fn create_group(db: &DbConn, payload: NewGroup) -> Result<groups::Model, DbErr> {
        groups::ActiveModel {
            name: Set(payload.name),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()
    }

    pub async fn update_group_by_id(
        db: &DbConn,
        id: i32,
        payload: UpdateGroup,
    ) -> Result<groups::Model, DbErr> {
        let mut group: groups::ActiveModel = Group::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find group.".to_owned()))
            .map(Into::into)?;

        group.name = Set(payload.name);
        group.updated_at = Set(Local::now().naive_local());

        group.update(db).await?.try_into_model()
    }

    // delete group by id
    pub async fn delete_group_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Group::delete_by_id(id).exec(db).await
    }

    // create config
    pub async fn create_config(db: &DbConn, form: NewConfig) -> Result<configs::Model, DbErr> {
        configs::ActiveModel {
            group_id: Set(form.group_id),
            name: Set(form.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()
    }

    pub async fn update_config_by_id(
        db: &DbConn,
        id: i32,
        payload: UpdateConfig,
    ) -> Result<configs::Model, DbErr> {
        let mut config = configs::ActiveModel::new();

        config.id = Set(id);
        if let Some(group_id) = payload.group_id {
            config.group_id = Set(group_id);
        }
        if let Some(name) = payload.name {
            config.name = Set(name);
        }

        if let Some(value) = payload.values {
            config.values = Set(value);
        }

        config.updated_at = Set(Local::now().naive_local());

        config.update(db).await?.try_into_model()
    }

    pub async fn delete_config_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Config::delete_by_id(id).exec(db).await
    }

    pub async fn create_user(
        db: &DbConn,
        email: String,
        password: String,
        nickname: String,
    ) -> Result<users::Model, DbErr> {
        users::ActiveModel {
            email: Set(email),
            password: Set(password),
            nickname: Set(nickname),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()
    }

    pub async fn create_template(
        db: &DbConn,
        payload: NewTemplate,
    ) -> Result<templates::Model, DbErr> {
        templates::ActiveModel {
            name: Set(payload.name),
            brief: Set(payload.brief),
            repo: Set(payload.repo),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()
    }

    pub async fn update_template_by_id(
        db: &DbConn,
        id: i32,
        payload: UpdateTemplate,
    ) -> Result<templates::Model, DbErr> {
        let mut template = templates::ActiveModel::new();

        template.id = Set(id);

        if let Some(name) = payload.name {
            template.name = Set(name);
        }

        if let Some(brief) = payload.brief {
            template.brief = Set(brief);
        }

        if let Some(tags) = payload.tags {
            template.tags = Set(Some(tags));
        }

        if let Some(repo) = payload.repo {
            template.repo = Set(repo);
        }

        template.updated_at = Set(Local::now().naive_local());
        template.update(db).await?.try_into_model()
    }

    // delete template by id
    pub async fn delete_template_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Template::delete_by_id(id).exec(db).await
    }

    // create project
    pub async fn create_project(
        db: &DbConn,
        payload: NewProject,
        user_id: i32,
    ) -> Result<ProjectVo, DbErr> {
        let project = projects::ActiveModel {
            name: Set(payload.name),
            repo: Set(payload.repo),
            repo_id: Set(payload.repo_id),
            description: Set(payload.description),
            user_id: Set(user_id),
            ..Default::default()
        }
        .save(db)
        .await?
        .try_into_model()?;

        let user = project.find_related(User).one(db).await?;

        match user {
            Some(user) => Ok(ProjectVo::new(project, user.nickname)),
            None => Err(DbErr::Custom("Failed to find user".to_string())),
        }
    }

    // update project by id
    pub async fn update_project_by_id(
        db: &DbConn,
        id: i32,
        payload: UpdateProject,
    ) -> Result<ProjectVo, DbErr> {
        let mut project = projects::ActiveModel::new();

        project.id = Set(id);

        if let Some(name) = payload.name {
            project.name = Set(name);
        }

        if let Some(description) = payload.description {
            project.description = Set(description);
        }

        project.updated_at = Set(Local::now().naive_local());
        let project = project.update(db).await?.try_into_model()?;

        let user = project.find_related(User).one(db).await?;

        match user {
            Some(user) => Ok(ProjectVo::new(project, user.nickname)),
            None => Err(DbErr::Custom("Failed to find user".to_string())),
        }
    }

    // delete project by id
    pub async fn delete_project_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Project::delete_by_id(id).exec(db).await
    }
}
