use sea_orm_migration::prelude::*;
use crate::m20230104_141642_create_groups;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Config::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Config::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Config::GroupId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("groups_configs")
                        .from(Config::Table, Config::GroupId)
                        .to(m20230104_141642_create_groups::Group::Table, m20230104_141642_create_groups::Group::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(Config::Name)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Config::Values)
                            .json()
                            .default("[]")
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Config::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Custom("NOW()".to_string()))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Config::UpdatedAt)
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
            .drop_table(Table::drop().table(Config::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Config {
    #[iden = "configs"]
    Table,
    Id,
    GroupId,
    Name,
    Values,
    CreatedAt,
    UpdatedAt,
}
