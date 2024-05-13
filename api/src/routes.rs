use crate::{
    handlers::{
        books::*, borrow::*, emails::*, index::*, login::*, logout::*, not_found, search::*,
        users::*, reload_templates, background::background_handler,
    },
    permission::Permission,
};

use actix_files::Files;
use actix_web::web;
use entity::AccessPermission;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.default_service(web::route().to(not_found))
        .service(Files::new("/static", "./api/static"))
        .route("/", web::get().to(index_handler))
        .route("/bg", web::get().to(background_handler))
        .service(
            web::resource("/login")
                .route(web::get().to(login_handler))
                .route(web::post().to(login_post_handler)),
        )
        .service(
            web::scope("/search")
                .wrap(Permission::new(AccessPermission::User))
                .route("", web::get().to(search_handler))
                .route("/s", web::get().to(search_get_handler)),
        )
        .service(
            web::scope("/control")
                .wrap(Permission::new(AccessPermission::Admin))
                .route("/reload_templates", web::get().to(reload_templates)),
        )
        .route("/logout", web::get().to(logout_handler))
        .service(
            web::resource("/register")
                .route(web::get().to(register_handler))
                .route(web::post().to(register_post_handler)),
        )
        .service(
            web::scope("/users")
                .service(
                    web::resource("")
                        .wrap(Permission::new(AccessPermission::Admin))
                        .route(web::get().to(list_users_handler)),
                )
                .service(
                    web::resource("/delete/{user_id}")
                        .wrap(Permission::new(AccessPermission::Admin))
                        .route(web::get().to(delete_user_handler)),
                )
                .wrap(Permission::new(AccessPermission::User))
                .route("/{user_id}", web::get().to(user_detail_handler)),
        )
        .service(
            web::scope("/borrow")
                .wrap(Permission::new(AccessPermission::Admin))
                .route("", web::get().to(list_borrowed_books_handler))
                .route("/delete/{borrow_id}", web::get().to(return_book_handler))
                .route("/{book_id}", web::post().to(borrow_book_post_handler)),
        )
        .service(
            web::scope("/books")
                .wrap(Permission::new(AccessPermission::User))
                .route("", web::get().to(list_books_handler))
                .service(
                    web::resource("/edit/{book_id}")
                        .wrap(Permission::new(AccessPermission::Admin))
                        .route(web::get().to(edit_book_handler))
                        .route(web::post().to(edit_book_post_handler)),
                )
                .service(
                    web::resource("/delete/{book_id}")
                        .wrap(Permission::new(AccessPermission::Admin))
                        .route(web::get().to(delete_book_handler)),
                )
                .service(
                    web::resource("/new")
                        .wrap(Permission::new(AccessPermission::Admin))
                        .route(web::get().to(new_book_handler))
                        .route(web::post().to(new_book_post_handler)),
                )
                .route("/{book_id}", web::get().to(book_detail_handler)),
        )
        .service(
            web::scope("/emails")
                .wrap(Permission::new(AccessPermission::User))
                .route("", web::get().to(list_emails_handler))
                .service(
                    web::resource("/send")
                        .route(web::get().to(new_email_handler))
                        .route(web::post().to(new_email_post_handler)),
                )
                .route("/delete/{email_id}", web::get().to(delete_email_handler))
                .route("/sent_mail", web::get().to(sent_email_handler))
                .route("/{email_id}", web::get().to(email_detail_handler)),
        );
}
