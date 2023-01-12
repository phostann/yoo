pub use sea_orm_migration::prelude::*;

mod m20230104_141642_create_groups;
mod m20230104_172456_create_configs;
mod m20230105_152813_create_users;
mod m20230106_093504_create_templates;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230104_141642_create_groups::Migration),
            Box::new(m20230104_172456_create_configs::Migration),
            Box::new(m20230105_152813_create_users::Migration),
            Box::new(m20230106_093504_create_templates::Migration),
        ]
    }
}
