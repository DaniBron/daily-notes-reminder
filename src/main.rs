use std::collections::HashMap;

use daily_notes_reminder::common::email::Email;

use daily_notes_reminder::common::topics_generator::TopicsGenerator;
use daily_notes_reminder::common::topics_generator_factory::topics_gen_factory;

use daily_notes_reminder::common::email_factory::email_factory;

use daily_notes_reminder::common::subject_generator_factory::subject_gen_factory;
use daily_notes_reminder::common::subject_generator::SubjectGenerator;


fn main() {
    if let Err(e) = run(){
        eprintln!("Application error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_path:  &str = "coding_notes.txt";

    let topics:HashMap<String, Vec<String>> = match topics_gen_factory(file_path) {
        Some(factory) => factory.generate_topics(file_path)?,
        None => return Err("Unsupported file type".into()),
    };

    let email_handler = email_factory();
    let email_content = subject_gen_factory();

    let _ = send_email(&topics, &email_handler, &email_content);

    Ok(())
}

fn send_email(topics: &HashMap<String, Vec<String>>, email_handler: &impl Email, subject_generator: &impl SubjectGenerator) -> Result<(), Box<dyn std::error::Error>> {
    
    let subject:    &str = "Daily Code Principles";
    
    let email_content: String = subject_generator.generate_subject(&topics)?;

    email_handler.send_email(subject, email_content)?;

    Ok(())
}