use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // info on ColumnDef: https://docs.rs/sea-orm-migration/latest/sea_orm_migration/prelude/struct.ColumnDef.html
                    // reference to column types across rust and sqlite: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
                    .col(ColumnDef::new(Tag::Name).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tag {
    Table,
    Id,
    Name
}
