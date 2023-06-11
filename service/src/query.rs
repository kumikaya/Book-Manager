use ::entity::{
    books, borrowed_books, emails, users, AccessPermission, BorrowedBooksResult,
    BorrowedBooksResultForBook, BorrowedBooksResultForUser, Email, IdResult,
};
use paste::paste;
use sea_orm::{
    sea_query::{Alias, Expr, SimpleExpr},
    *,
};

pub struct Query;

macro_rules! basic_query_def {
    ($name:ident) => {
        paste!{
            pub async fn [<find_ $name _by_id>]<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<[<$name s>]::Model>, DbErr> {
                [<$name s>]::Entity::find_by_id(id).one(db).await
            }

            pub async fn [<find_ $name s_in_page>]<C: ConnectionTrait>(
                db: &C,
                page: u64,
                number_per_page: u64,
            ) -> Result<(Vec<[<$name s>]::Model>, u64), DbErr> {
                // Setup paginator
                let paginator = [<$name s>]::Entity::find()
                    .order_by_asc([<$name s>]::Column::Id)
                    .paginate(db, number_per_page);
                let num_pages = paginator.num_pages().await?;

                // Fetch paginated posts
                paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
            }
        }
    };
}

macro_rules! query_by_field_def {
    ($name:ident, $field:ident) => {
        paste!{
            pub async fn [<find_ $name s_by_ $field>]<C: ConnectionTrait, V: Into<Value>>(db: &C, value: V) -> Result<Vec<[<$name s>]::Model>, DbErr> {
                [<$name s>]::Entity::find().filter([<$name s>]::Column::[<$field:camel>].eq(value)).all(db).await
            }
        }
    };
}

macro_rules! query_by_field_unique_def {
    ($name:ident, $field:ident) => {
        paste!{
            pub async fn [<find_ $name _by_ $field>]<C: ConnectionTrait, V: Into<Value>>(db: &C, value: V) -> Result<Option<[<$name s>]::Model>, DbErr> {
                [<$name s>]::Entity::find().filter([<$name s>]::Column::[<$field:camel>].eq(value)).one(db).await
            }
        }
    };
}

impl Query {
    basic_query_def!(book);
    basic_query_def!(user);
    basic_query_def!(borrowed_book);
    basic_query_def!(email);
    query_by_field_unique_def!(user, name);
    query_by_field_def!(book, name);
    query_by_field_def!(book, author);
    query_by_field_def!(borrowed_book, user_id);
    query_by_field_def!(borrowed_book, book_id);
    query_by_field_def!(email, sender_id);
    query_by_field_def!(email, recipient_id);

    pub async fn find_admin_ids<C: ConnectionTrait>(db: &C) -> Result<Vec<i32>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Permission.eq(AccessPermission::Admin))
            .into_model::<IdResult>()
            .all(db)
            .await
            .map(|ids| ids.into_iter().map(|id| id.id).collect())
    }

    pub async fn find_user_ids<C: ConnectionTrait>(db: &C) -> Result<Vec<i32>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Permission.eq(AccessPermission::User))
            .into_model::<IdResult>()
            .all(db)
            .await
            .map(|ids| ids.into_iter().map(|id| id.id).collect())
    }

    pub async fn find_email_detail_by_id<C: ConnectionTrait>(
        db: &C,
        email_id: i32,
    ) -> Result<Option<Email>, DbErr> {
        let join_clouse = |id_column: emails::Column| {
            emails::Entity::belongs_to(users::Entity)
                .from(id_column)
                .to(users::Column::Id)
                .into()
        };

        let recipient_name = Alias::new("recipient");

        emails::Entity::find()
            .column_as(
                Into::<SimpleExpr>::into(Expr::col((recipient_name.clone(), users::Column::Name))),
                "recipient_name",
            )
            .column_as(users::Column::Name, "sender_name")
            .filter(emails::Column::Id.eq(email_id))
            .join_as(
                JoinType::InnerJoin,
                join_clouse(emails::Column::RecipientId),
                recipient_name,
            )
            .join(JoinType::InnerJoin, join_clouse(emails::Column::SenderId))
            .order_by_asc(emails::Column::DateTime)
            .into_model::<Email>()
            .one(db)
            .await
    }

    pub async fn find_borrowed_books_detail_by_user_id<C: ConnectionTrait>(
        db: &C,
        user_id: i32,
    ) -> Result<Vec<BorrowedBooksResultForBook>, DbErr> {
        borrowed_books::Entity::find()
            .column_as(borrowed_books::Column::Id, "borrow_id")
            .column_as(books::Column::Name, "book_name")
            .column_as(books::Column::Author, "book_author")
            .column_as(books::Column::Isbn, "isbn")
            .filter(borrowed_books::Column::UserId.eq(user_id))
            .join(
                JoinType::InnerJoin,
                borrowed_books::Entity::belongs_to(books::Entity)
                    .from(borrowed_books::Column::BookId)
                    .to(books::Column::Id)
                    .into(),
            )
            .into_model::<BorrowedBooksResultForBook>()
            .all(db)
            .await
    }

    pub async fn find_borrowed_books_detail_by_book_id<C: ConnectionTrait>(
        db: &C,
        user_id: i32,
    ) -> Result<Vec<BorrowedBooksResultForUser>, DbErr> {
        borrowed_books::Entity::find()
            .column_as(borrowed_books::Column::Id, "borrow_id")
            .column_as(users::Column::Name, "user_name")
            .column_as(users::Column::Nickname, "user_nickname")
            .filter(borrowed_books::Column::BookId.eq(user_id))
            .join(
                JoinType::InnerJoin,
                borrowed_books::Entity::belongs_to(users::Entity)
                    .from(borrowed_books::Column::UserId)
                    .to(users::Column::Id)
                    .into(),
            )
            .order_by_asc(borrowed_books::Column::ReturnDate)
            .into_model::<BorrowedBooksResultForUser>()
            .all(db)
            .await
    }

    pub async fn find_emails_in_page_by_sender_id<C: ConnectionTrait>(
        db: &C,
        sender_id: i32,
        page: u64,
        number_per_page: u64,
    ) -> Result<(Vec<Email>, u64), DbErr> {
        find_emails_in_page(db, sender_id, true, page, number_per_page).await
    }

    pub async fn find_emails_in_page_by_recipient_id<C: ConnectionTrait>(
        db: &C,
        recipient_id: i32,
        page: u64,
        number_per_page: u64,
    ) -> Result<(Vec<Email>, u64), DbErr> {
        find_emails_in_page(db, recipient_id, false, page, number_per_page).await
    }

    pub async fn find_borrowed_books_detail_in_page<C: ConnectionTrait>(
        db: &C,
        page: u64,
        number_per_page: u64,
    ) -> Result<(Vec<BorrowedBooksResult>, u64), DbErr> {
        find_borrowed_books_in_page(db, page, number_per_page).await
    }

    pub async fn find_books_by_keyword_in_page<C: ConnectionTrait>(
        db: &C,
        keyword: &str,
        page: u64,
        number_per_page: u64,
    ) -> Result<(Vec<books::Model>, u64), DbErr> {
        let pattern = format!("%{}%", keyword);
        let paginator = books::Entity::find()
            .filter(
                books::Column::Name
                    .like(&pattern)
                    .or(books::Column::Isbn.like(&pattern))
                    .or(books::Column::Author.like(&pattern))
                    .or(books::Column::Publisher.like(&pattern)),
            )
            .order_by_asc(books::Column::Name)
            .paginate(db, number_per_page);
        let num_pages = paginator.num_pages().await?;
        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_users_by_keyword_in_page<C: ConnectionTrait>(
        db: &C,
        keyword: &str,
        page: u64,
        number_per_page: u64,
    ) -> Result<(Vec<users::Model>, u64), DbErr> {
        let pattern = format!("%{}%", keyword);
        let paginator = users::Entity::find()
            .filter(
                users::Column::Name
                    .like(&pattern)
                    .or(users::Column::Nickname.like(&pattern)),
            )
            .order_by_asc(users::Column::Name)
            .paginate(db, number_per_page);
        let num_pages = paginator.num_pages().await?;
        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}

pub async fn find_emails_in_page<C: ConnectionTrait>(
    db: &C,
    user_id: i32,
    is_sender: bool,
    page: u64,
    number_per_page: u64,
) -> Result<(Vec<Email>, u64), DbErr> {
    // Setup paginator
    let (id_column, delete_column) = if is_sender {
        (emails::Column::SenderId, emails::Column::DeletedBySender)
    } else {
        (
            emails::Column::RecipientId,
            emails::Column::DeletedByRecipient,
        )
    };
    let join_clouse = |id_column: emails::Column| {
        emails::Entity::belongs_to(users::Entity)
            .from(id_column)
            .to(users::Column::Id)
            .into()
    };

    let recipient_name = Alias::new("recipient");

    let paginator = emails::Entity::find()
        .column_as(
            Into::<SimpleExpr>::into(Expr::col((recipient_name.clone(), users::Column::Name))),
            "recipient_name",
        )
        .column_as(users::Column::Name, "sender_name")
        .filter(id_column.eq(user_id).and(delete_column.eq(false)))
        .join_as(
            JoinType::InnerJoin,
            join_clouse(emails::Column::RecipientId),
            recipient_name,
        )
        .join(JoinType::InnerJoin, join_clouse(emails::Column::SenderId))
        .order_by_desc(emails::Column::DateTime)
        .into_model::<Email>()
        .paginate(db, number_per_page);
    let num_pages = paginator.num_pages().await?;

    // Fetch paginated posts
    paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
}

pub async fn find_borrowed_books_in_page<C: ConnectionTrait>(
    db: &C,
    page: u64,
    number_per_page: u64,
) -> Result<(Vec<BorrowedBooksResult>, u64), DbErr> {
    let paginator = borrowed_books::Entity::find()
        .column_as(borrowed_books::Column::Id, "borrow_id")
        .column_as(users::Column::Name, "user_name")
        .column_as(users::Column::Nickname, "user_nickname")
        .column_as(books::Column::Name, "book_name")
        .column_as(books::Column::Isbn, "isbn")
        .join(
            JoinType::InnerJoin,
            borrowed_books::Entity::belongs_to(books::Entity)
                .from(borrowed_books::Column::BookId)
                .to(books::Column::Id)
                .into(),
        )
        .join(
            JoinType::InnerJoin,
            borrowed_books::Entity::belongs_to(users::Entity)
                .from(borrowed_books::Column::UserId)
                .to(users::Column::Id)
                .into(),
        )
        .order_by_desc(borrowed_books::Column::BorrowDate)
        .into_model::<BorrowedBooksResult>()
        .paginate(db, number_per_page);
    let num_pages = paginator.num_pages().await?;

    // Fetch paginated posts
    paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
}
