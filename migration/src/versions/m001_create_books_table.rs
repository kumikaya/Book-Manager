use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookFields::Books)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BookFields::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookFields::Name).string().not_null())
                    .col(ColumnDef::new(BookFields::Author).string().not_null())
                    .col(ColumnDef::new(BookFields::Publisher).string().not_null())
                    .col(ColumnDef::new(BookFields::PublishYear).date().not_null())
                    .col(
                        ColumnDef::new(BookFields::Isbn)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BookFields::Copies).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookFields::Books).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(super) enum BookFields {
    Books,
    Id,
    Name,
    Author,
    Publisher,
    PublishYear,
    Isbn,
    Copies,
}
