use ::entity::{
    configs, configs::Entity as Config, groups, groups::Entity as Group, templates,
    templates::Entity as Template, users, users::Entity as User,
};
use sea_orm::{
    sea_query::{Expr, PgFunc},
    DeriveColumn, EnumIter, *,
};
use std::collections::HashSet;

use crate::{ConfigFilter, GroupFilter, TemplateFilter};

pub struct Query;

impl Query {
    pub async fn get_group_by_id(
        db: &DbConn,
        group_id: i32,
    ) -> Result<Option<groups::Model>, DbErr> {
        Group::find_by_id(group_id).one(db).await
    }

    pub async fn list_groups(
        db: &DbConn,
        page: u64,
        page_size: u64,
        filter: GroupFilter,
    ) -> Result<(Vec<groups::Model>, u64, u64), DbErr> {
        let mut query = Group::find().order_by_desc(groups::Column::UpdatedAt);

        if let Some(name) = filter.name {
            query = query.filter(groups::Column::Name.contains(name.as_ref()));
        }

        let paginator = query.paginate(db, page_size);

        let total = paginator.num_items().await?;
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, total, num_pages))
    }

    pub async fn get_config_by_id(
        db: &DbConn,
        config_id: i32,
    ) -> Result<Option<configs::Model>, DbErr> {
        Config::find_by_id(config_id).one(db).await
    }

    pub async fn get_config_by_group_id(db: &DbConn, group_id: i32) -> Result<u64, DbErr> {
        Config::find()
            .filter(configs::Column::GroupId.eq(group_id))
            .count(db)
            .await
    }

    pub async fn list_configs(
        db: &DbConn,
        page: u64,
        page_size: u64,
        filter: ConfigFilter,
    ) -> Result<(Vec<configs::Model>, u64, u64), DbErr> {
        let mut query = Config::find();

        if let Some(group_id) = filter.group_id {
            query = query.filter(configs::Column::GroupId.eq(group_id));
        }

        if let Some(name) = filter.name {
            query = query.filter(configs::Column::Name.contains(name.as_ref()));
        }

        let paginator = query.paginate(db, page_size);

        let total = paginator.num_items().await?;

        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, total, num_pages))
    }

    pub async fn find_user_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        User::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn list_templates(
        db: &DbConn,
        page: u64,
        page_size: u64,
        filter: TemplateFilter,
    ) -> Result<(Vec<templates::Model>, u64, u64), DbErr> {
        let mut query = Template::find();

        if let Some(name) = filter.name {
            query = query.filter(templates::Column::Name.contains(name.as_ref()));
        }

        if let Some(kind) = filter.kind {
            query = query.filter(templates::Column::Kind.eq(kind));
        }

        if let Some(tag) = filter.tag {
            query = query.filter(Expr::eq(
                Expr::val(tag),
                Expr::expr(PgFunc::any(Expr::col(templates::Column::Tags))),
            ))
        }

        let paginator = query.paginate(db, page_size);

        let total = paginator.num_items().await?;
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, total, num_pages))
    }

    pub async fn get_template_by_id(
        db: &DbConn,
        template_id: i32,
    ) -> Result<Option<templates::Model>, DbErr> {
        Template::find_by_id(template_id).one(db).await
    }

    pub async fn get_template_tags(db: &DbConn) -> Result<HashSet<String>, DbErr> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum ResultCol {
            Tags,
        }

        let res: Vec<Vec<String>> = Template::find()
            .select_only()
            .column(templates::Column::Tags)
            .into_values::<_, ResultCol>()
            .all(db)
            .await?;

        let res = res.into_iter().flatten().collect::<HashSet<String>>();

        Ok(res)
    }
}
