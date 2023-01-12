use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Template::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Template::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Template::Name).string().string_len(50).unique_key().not_null())
                    .col(ColumnDef::new(Template::Repo).string().string_len(255).unique_key().not_null())
                    .col(ColumnDef::new(Template::Brief).string().string_len(50).not_null())
                    .col(ColumnDef::new(Template::Kind).string().string_len(20).not_null())
                    .col(ColumnDef::new(Template::Tags).array(ColumnType::String(None)).default("{}"))
                    .col(ColumnDef::new(Template::CreatedAt).timestamp().default(SimpleExpr::Custom("NOW()".to_string())).not_null())
                    .col(ColumnDef::new(Template::UpdatedAt).timestamp().default(SimpleExpr::Custom("NOW()".to_string())).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Template::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Template {
    #[iden = "templates"]
    Table,
    Id,
    Name,
    Repo,
    Brief,
    Tags,
    Kind,
    CreatedAt,
    UpdatedAt,
}
