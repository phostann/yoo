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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .string_len(100)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Nickname)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Custom("NOW()".to_string()))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    #[iden = "users"]
    Table,
    Id,
    Email,
    Nickname,
    Password,
    CreatedAt,
    UpdatedAt,
}
