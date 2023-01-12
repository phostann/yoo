use sea_orm_migration::prelude::*;
// use sea_query::SimpleExpr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Group::Name)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Group::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Custom("NOW()".to_string()))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Group::UpdatedAt)
                            .timestamp()
                            .default(SimpleExpr::Custom("NOW()".to_string()))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Group {
    #[iden = "groups"]
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
