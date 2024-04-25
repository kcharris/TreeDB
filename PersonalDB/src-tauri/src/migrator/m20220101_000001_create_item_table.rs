use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// code may be hanlded by the above derive
// impl MigrationName for Migration {
//     fn name(&self) -> &str {
//         "m20220101_000001_create_item_table" // Make sure this matches with the file name
//     }
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // info on ColumnDef: https://docs.rs/sea-orm-migration/latest/sea_orm_migration/prelude/struct.ColumnDef.html
                    // reference to column types across rust and sqlite: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
                    .col(ColumnDef::new(Item::Parent).integer())
                    .col(ColumnDef::new(Item::Name).text().not_null())
                    .col(ColumnDef::new(Item::Availability).text())
                    .col(ColumnDef::new(Item::Priority).integer())

                    .col(ColumnDef::new(Item::Completed).boolean())
                    .col(ColumnDef::new(Item::Resource).text())
                    .col(ColumnDef::new(Item::EstTime).integer())

                    .col(ColumnDef::new(Item::StartDate).text())
                    .col(ColumnDef::new(Item::EndDate).text())
                    .col(ColumnDef::new(Item::Description).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Item {
    Table,
    Id,
    Priority,

    Parent,
    Name,
    Availability,

    Completed,
    Resource,
    #[sea_orm(iden = "est_time")]
    EstTime,

    #[sea_orm(iden = "start_date")]
    StartDate,
    #[sea_orm(iden = "end_date")]
    EndDate,
    Description    
}
