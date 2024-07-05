use sea_orm_migration::prelude::*;

pub struct Migrator;
pub mod m20220101_000001_create_item_table;
pub mod m20220101_000002_create_tag_table;
pub mod m20220101_000003_create_item_tag_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_item_table::Migration),
            Box::new(m20220101_000002_create_tag_table::Migration),
            Box::new(m20220101_000003_create_item_tag_table::Migration)
        ]
    }
}