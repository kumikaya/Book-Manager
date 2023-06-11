pub mod list;
pub mod send;
pub mod delete;
pub mod read;
pub mod sent_mail;

use entity::emails;
pub use list::*;
pub use send::*;
pub use delete::*;
pub use read::*;
pub use sent_mail::*;

enum EmailAccess {
    SenderAndRecipient,
    Sender,
    Recipient,
    Unrelated,
}

fn get_email_access(email: &emails::Model, user_id: i32) -> EmailAccess {
    let on_sender = email.sender_id == user_id && !email.deleted_by_sender;
    let on_recipient = email.recipient_id == user_id && !email.deleted_by_recipient;
    if on_sender && on_recipient {
        EmailAccess::SenderAndRecipient
    } else if on_sender {
        EmailAccess::Sender
    } else if on_recipient {
        EmailAccess::Recipient
    } else {
        EmailAccess::Unrelated
    }
}