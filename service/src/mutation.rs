use ::entity::{books, borrowed_books, emails, users, AccessPermission, EmailCategory};
use chrono::NaiveDate;
use paste::paste;
use sea_orm::*;

pub struct Mutation;

// macro_rules! update_by_id_def {
//     ($name:ident, $field:ident) => {
//         paste! {
//             pub async fn [<update_ $name _ $field _by_id>](
//                 db: &DbConn,
//                 id: i32,
//                 form_data: [<$name s>]::Model,
//             ) -> Result<[<$name s>]::Model, DbErr> {
//                 let new_data: [<$name s>]::ActiveModel = [<$name s>]::Entity::find_by_id(id)
//                     .one(db)
//                     .await?
//                     .ok_or(DbErr::Custom(format!("Cannot find {}.", stringify!($name))))
//                     .map(Into::into)?;

//                 [<$name s>]::ActiveModel {
//                     id: new_data.id,
//                     $field: Set(form_data.$field.to_owned()),
//                     ..Default::default()
//                 }
//                 .update(db)
//                 .await
//             }
//         }
//     };
// }

macro_rules! delete_by_id_def {
    ($name:ident) => {
        paste! {
            pub async fn [<delete_ $name>]<C: ConnectionTrait>(db: &C, id: i32) -> Result<DeleteResult, DbErr> {
                let entity: [<$name s>]::ActiveModel = [<$name s>]::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or(DbErr::Custom(format!("Cannot find {}.", stringify!($name))))
                    .map(Into::into)?;

                entity.delete(db).await
            }
        }
    };
}

impl Mutation {
    pub async fn create_user<C: ConnectionTrait>(
        db: &C,
        username: String,
        nickname: String,
        password_hash: String,
        permission: AccessPermission,
    ) -> Result<users::Model, DbErr> {
        let registration_date = chrono::Local::now().naive_local().date();
        users::ActiveModel {
            name: Set(username),
            nickname: Set(nickname),
            password_hash: Set(password_hash),
            permission: Set(permission),
            registration_date: Set(registration_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_user_by_id<C: ConnectionTrait>(
        db: &C,
        id: i32,
        nickname: String,
        password_hash: String,
    ) -> Result<users::Model, DbErr> {
        let new_data: users::ActiveModel = users::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find user.")))
            .map(Into::into)?;
        users::ActiveModel {
            id: new_data.id,
            nickname: Set(nickname),
            password_hash: Set(password_hash),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn create_book<C: ConnectionTrait>(
        db: &C,
        form_data: books::Model,
    ) -> Result<books::Model, DbErr> {
        let books::Model {
            name,
            author,
            publisher,
            publish_year,
            isbn,
            copies,
            ..
        } = form_data;
        books::ActiveModel {
            name: Set(name),
            author: Set(author),
            publisher: Set(publisher),
            publish_year: Set(publish_year),
            isbn: Set(isbn),
            copies: Set(copies),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_book_by_id<C: ConnectionTrait>(
        db: &C,
        id: i32,
        form_data: books::Model,
    ) -> Result<books::Model, DbErr> {
        let new_data: books::ActiveModel = books::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find book.")))
            .map(Into::into)?;
        let books::Model {
            name,
            author,
            publisher,
            publish_year,
            isbn,
            copies,
            ..
        } = form_data;
        books::ActiveModel {
            id: new_data.id,
            name: Set(name),
            author: Set(author),
            publisher: Set(publisher),
            publish_year: Set(publish_year),
            isbn: Set(isbn),
            copies: Set(copies),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn update_book_copies_by_id<C: ConnectionTrait>(
        db: &C,
        id: i32,
        copies: i32,
    ) -> Result<books::Model, DbErr> {
        let new_data: books::ActiveModel = books::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find book.")))
            .map(Into::into)?;
        books::ActiveModel {
            id: new_data.id,
            copies: Set(copies),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn create_borrowed_book<C: ConnectionTrait>(
        db: &C,
        user_id: i32,
        book_id: i32,
        borrow_date: NaiveDate,
        return_date: NaiveDate,
    ) -> Result<borrowed_books::Model, DbErr> {
        borrowed_books::ActiveModel {
            user_id: Set(user_id),
            book_id: Set(book_id),
            borrow_date: Set(borrow_date),
            return_date: Set(return_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn create_email<C: ConnectionTrait>(
        db: &C,
        category: EmailCategory,
        sender_id: i32,
        recipient_id: i32,
        subject: String,
        content: String,
    ) -> Result<emails::Model, DbErr> {
        let date_time = chrono::Local::now().naive_local();
        emails::ActiveModel {
            category: Set(category),
            sender_id: Set(sender_id),
            recipient_id: Set(recipient_id),
            subject: Set(subject),
            content: Set(content),
            date_time: Set(date_time),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    // update_by_id_def!(user, name);
    // update_by_id_def!(user, password);
    // update_by_id_def!(book, copies);
    // update_by_id_def!(borrowed_book, return_date);

    delete_by_id_def!(user);
    delete_by_id_def!(book);
    delete_by_id_def!(borrowed_book);
    delete_by_id_def!(email);

    pub async fn delete_email_by_id_on_sender<C: ConnectionTrait>(
        db: &C,
        id: i32,
    ) -> Result<(), DbErr> {
        delete_email_by_id_weak(db, id, true).await
    }

    pub async fn delete_email_by_id_on_recipient<C: ConnectionTrait>(
        db: &C,
        id: i32,
    ) -> Result<(), DbErr> {
        delete_email_by_id_weak(db, id, false).await
    }
}

pub async fn delete_email_by_id_weak<C: ConnectionTrait>(
    db: &C,
    id: i32,
    on_sender: bool,
) -> Result<(), DbErr> {
    let entity: emails::ActiveModel = emails::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom(format!("Cannot find email.")))
        .map(Into::into)?;

    let data = if on_sender {
        emails::ActiveModel {
            deleted_by_sender: Set(true),
            ..entity
        }
    } else {
        emails::ActiveModel {
            deleted_by_recipient: Set(true),
            ..entity
        }
    }
    .update(db)
    .await?;
    if data.deleted_by_sender && data.deleted_by_recipient {
        Mutation::delete_email(db, id).await?;
    }
    Ok(())
}
