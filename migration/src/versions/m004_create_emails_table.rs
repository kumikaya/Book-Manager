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
                    .table(EmailsFields::Emails)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailsFields::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::Category)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::SenderId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::RecipientId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::Subject)
                            .string_len(MAX_EMAIL_TITLE_LEN)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::Content)
                            .string_len(MAX_EMAIL_CONTENT_LEN)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::DateTime)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::DeletedByRecipient)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(EmailsFields::DeletedBySender)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_addr_user_id")
                            .from(
                                EmailsFields::Emails,
                                EmailsFields::SenderId,
                            )
                            .to(UserFields::Users, UserFields::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_recv_user_id")
                            .from(
                                EmailsFields::Emails,
                                EmailsFields::RecipientId,
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
                    .table(EmailsFields::Emails)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(super) enum EmailsFields {
    Emails,
    Id,
    Category,
    SenderId,
    RecipientId,
    Subject,
    Content,
    DateTime,
    DeletedByRecipient,
    DeletedBySender,
}
