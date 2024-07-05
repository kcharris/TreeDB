use sea_orm_migration::prelude::*;
use crate::migrator::m20220101_000002_create_tag_table::Tag;
use crate::migrator::m20220101_000001_create_item_table::Item;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE `item_tag` (
                `item_id` int NOT NULL,
                `tag_id` int NOT NULL,
                FOREIGN KEY (`item_id`) REFERENCES `item` (`id`) ON DELETE CASCADE,
                FOREIGN KEY (`tag_id`) REFERENCES `tag` (`id`) ON DELETE CASCADE,
                PRIMARY KEY (`item_id`, `tag_id`)
            )"
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ItemTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ItemTag {
    Table,
    #[sea_orm(iden = "item_id")]
    ItemId,
    #[sea_orm(iden = "tag_id")]
    TagId
}
