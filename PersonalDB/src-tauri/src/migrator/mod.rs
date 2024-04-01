use sea_orm_migration::prelude::*;

pub struct Migrator;
pub mod m20220101_000001_create_item_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_item_table::Migration)]
    }
}