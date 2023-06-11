use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFields::Users)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFields::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserFields::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserFields::Nickname).string().not_null())
                    .col(ColumnDef::new(UserFields::PasswordHash).string().not_null())
                    .col(ColumnDef::new(UserFields::Permission).integer().not_null())
                    .col(ColumnDef::new(UserFields::RegistrationDate).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFields::Users).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(super) enum UserFields {
    Users,
    Id,
    Name,
    Nickname,
    PasswordHash,
    Permission,
    RegistrationDate,
}
