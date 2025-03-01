use std::env;

use daily_notes_reminder::common::email::Email;

use daily_notes_reminder::common::topics_generator::TopicsGenerator;
use daily_notes_reminder::common::topics_generator::Topics;
use daily_notes_reminder::common::topics_generator_factory::topics_gen_factory;

use daily_notes_reminder::common::email_factory::email_factory;

use daily_notes_reminder::common::subject_generator_factory::subject_gen_factory;
use daily_notes_reminder::common::subject_generator::SubjectGenerator;

use daily_notes_reminder::s3_manager::S3Manager;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::Value;

const FILE: &str = "coding_notes.txt";
const FILE_PATH: &str = "/tmp/coding_notes.txt";

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(lambda_handler)).await
   }

async fn lambda_handler(event: LambdaEvent<Value>) -> Result<String, Error> {
    run().await.map_err(|e| Error::from(e.to_string()))?;
    Ok("Execution completed".to_string())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting");

    let s3_manager = S3Manager::new();

    let _ = match s3_manager.download_file(FILE).await {
        Ok(_) => println!("File downloaded"),
        Err(e) => eprintln!("Error: {}", e),
    };

    let topics = match topics_gen_factory(FILE_PATH) {
        Some(factory) => factory.generate_topics(FILE_PATH)?,
        None => return Err("Unsupported file type".into()),
    };

    let email_handler = email_factory();
    let email_content = subject_gen_factory();

    let _ = send_email(&topics, &email_handler, &email_content)?;

    Ok(())
}

fn send_email(topics: &Topics, email_handler: &impl Email, subject_generator: &impl SubjectGenerator) -> Result<(), Box<dyn std::error::Error>> {
    
    let subject:    &str = "Daily Code Principles";
    
    let email_content: String = subject_generator.generate_subject(&topics)?;

    email_handler.send_email(subject, email_content)?;

    Ok(())
}