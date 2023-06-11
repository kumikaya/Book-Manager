use super::m002_create_users_table::UserFields;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

const MAX_EMAIL_TITLE_LEN: u32 = 255;
const MAX_EMAIL_CONTENT_LEN: u32 = 1000;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailMessagesFields::EmailMessages)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailMessagesFields::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailMessagesFields::Category)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailMessagesFields::SenderId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailMessagesFields::Subject)
                            .string_len(MAX_EMAIL_TITLE_LEN)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailMessagesFields::Content)
                            .string_len(MAX_EMAIL_CONTENT_LEN)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailMessagesFields::Date)
                            .date()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_addr_user_id")
                            .from(
                                EmailMessagesFields::EmailMessages,
                                EmailMessagesFields::SenderId,
                            )
                            .to(UserFields::Users, UserFields::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(EmailMessagesFields::EmailMessages)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(super) enum EmailMessagesFields {
    EmailMessages,
    Id,
    SenderId,
    Category,
    Subject,
    Content,
    Date,
}
