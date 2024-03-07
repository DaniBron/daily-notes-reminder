use crate::common::email::Email;
use crate::email_handler::EmailHandler;

pub fn email_factory() -> impl Email {
    EmailHandler::new()
}