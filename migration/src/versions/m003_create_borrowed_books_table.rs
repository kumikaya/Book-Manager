use super::{m001_create_books_table::BookFields, m002_create_users_table::UserFields};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BorrowedBookFields::BorrowedBooks)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BorrowedBookFields::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BorrowedBookFields::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BorrowedBookFields::BookId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BorrowedBookFields::BorrowDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BorrowedBookFields::ReturnDate)
                            .date()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(
                                BorrowedBookFields::BorrowedBooks,
                                BorrowedBookFields::UserId,
                            )
                            .to(UserFields::Users, UserFields::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_id")
                            .from(
                                BorrowedBookFields::BorrowedBooks,
                                BorrowedBookFields::BookId,
                            )
                            .to(BookFields::Books, BookFields::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(BorrowedBookFields::BorrowedBooks)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(super) enum BorrowedBookFields {
    BorrowedBooks,
    Id,
    UserId,
    BookId,
    BorrowDate,
    ReturnDate,
}
