use rand::Rng;
use std::collections::HashMap;
use std::error::Error;

use crate::common::subject_generator::SubjectGenerator;

pub struct RandomSubjectGenerator {}

impl SubjectGenerator for RandomSubjectGenerator {

    fn generate_subject(&self, topics: &HashMap<String, Vec<String>>) -> Result<String, Box<dyn Error>> {
        //let mut topics = HashMap::new();
      
        Ok(self.generate_html_str(&topics))
    }

}

impl RandomSubjectGenerator {

    pub fn new() -> RandomSubjectGenerator {
        RandomSubjectGenerator {}
    }

    fn get_random_topic<'a>(&'a self, topics: &'a HashMap<String, Vec<String>>) -> (&String, &Vec<String>) {
        let mut rng = rand::thread_rng();

        let random_topic = rng.gen_range(0..topics.len());

        let topics_keys: Vec<_> = topics.iter().collect();

        return topics_keys[random_topic];
    }

    fn generate_html_str(&self, topics: &HashMap<String, Vec<String>>) -> String {
        let mut email_html: String = include_str!("styles/style.html").to_string();

        let (topic, bullet_points) = self.get_random_topic(&topics);

        email_html = email_html.replace("{{header}}", topic);

        let text: String = bullet_points
            .into_iter()
            .map(|paragraph| {
                let trimmed_paragraph = paragraph.trim();
                if trimmed_paragraph.is_empty() {
                    String::new()
                } else {
                    format!("<li>{}</li>\n", trimmed_paragraph) // Each paragraph becomes a list item
                }
            })
            .collect();

        email_html = email_html.replace("{{content}}", &text);

        return email_html;
    }
    
}
