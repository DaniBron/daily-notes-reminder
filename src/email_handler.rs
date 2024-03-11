//use tokio::task;
//use dotenv::dotenv;
use lettre::message::{header, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;
use std::env;

use crate::common::email::Email;

pub struct EmailHandler {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    sender_email: String,
    recipient_emails: Vec<String>,
}

impl Email for EmailHandler {
    fn send_email(&self, subject: &str, text: String) -> Result<(), Box<dyn std::error::Error>> {

        let emailes = self.emailes_to_send(subject, text);

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = SmtpTransport::relay(&self.smtp_server)
            .unwrap() // Unwrap the Result, panics in case of error
            .credentials(creds) // Provide the credentials to the transport
            .build();

        for (customer, email) in emailes.into_iter() {
            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully! to {}", customer),
                Err(e) => eprintln!("Could not send email to {}: {:?}", customer, e),
            }
        }

        Ok(())
    }
}

impl EmailHandler {
    pub fn new() -> Self {
        //dotenv().ok().expect(".env file can't be found!");

        // Read SMTP configuration from environment variables
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set in .env");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set in .env");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set in .env");
        let sender_email = env::var("SENDER_EMAIL").expect("SENDER_EMAIL not set in .env");

        // Read recipient emails from environment variable
        let recipient_emails_str =
            env::var("RECIPIENT_EMAILS").expect("RECIPIENT_EMAILS not set in .env");
        let recipient_emails = recipient_emails_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        EmailHandler {
            smtp_server,
            smtp_username,
            smtp_password,
            sender_email,
            recipient_emails,
        }
    }

    fn emailes_to_send(&self, subject: &str, text: String) -> HashMap<String, Message> {
        let mut emailes: HashMap<String, Message> = HashMap::new();

        for costumer in &self.recipient_emails {
            let email = Message::builder()
                // Set the sender's name and email address
                .from(self.sender_email.as_str().parse().unwrap())
                // Set the recipient's name and email address
                .to(costumer.parse().unwrap())
                // Set the subject of the email
                .subject(subject)
                // Set the body content of the email
                //.body(String::from(text))
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(text.clone()), // The body is set with the HTML content
                )
                .unwrap();

                emailes.insert(costumer.clone(), email);
        }

        emailes

    }

}
