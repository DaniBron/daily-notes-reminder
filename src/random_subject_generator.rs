use rand::Rng;
use crate::common::topics_generator::Topics;
use crate::common::topics_generator::BulletNode;
use std::error::Error;
use crate::common::subject_generator::SubjectGenerator;

pub struct RandomSubjectGenerator {}

impl SubjectGenerator for RandomSubjectGenerator {
    fn generate_subject(&self, topics: &Topics) -> Result<String, Box<dyn Error>> {
        Ok(self.generate_html_str(topics))
    }
}

impl RandomSubjectGenerator {
    pub fn new() -> RandomSubjectGenerator {
        RandomSubjectGenerator {}
    }

    fn get_random_topic<'a>(&'a self, topics: &'a Topics) -> Option<&BulletNode>{
        if topics.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_topic = rng.gen_range(0..topics.root_nodes.len());
        let topic: &BulletNode = &topics.root_nodes[random_topic]; //topics.iter().collect();
        
        Some(topic)
    }

    fn generate_html_str(&self, topics: &Topics) -> String {
        let mut email_html: String = include_str!("styles/style.html").to_string();

        // Get a random topic
        if let Some(topic) = self.get_random_topic(&topics) {
            // Set the header from the topic content
            email_html = email_html.replace("{{header}}", &topic.content);

            // Generate HTML for children nodes
            let mut content = String::new();
            
            // Process level 1 nodes (main bullet points)
            for child in &topic.children {
                if child.level == 1 {
                    content.push_str("<li>");
                    content.push_str(&child.content);
                    
                    // Check for level 2 children (sub-bullets)
                    let sub_bullets: Vec<&BulletNode> = child.children
                        .iter()
                        .filter(|node| node.level == 2)
                        .collect();
                    
                    if !sub_bullets.is_empty() {
                        content.push_str("<ul>");
                        for sub_bullet in sub_bullets {
                            content.push_str(&format!("<li>{}</li>", sub_bullet.content));
                        }
                        content.push_str("</ul>");
                    }
                    
                    content.push_str("</li>\n");
                }
            }

            email_html = email_html.replace("{{content}}", &content);
        } else {
            email_html = email_html.replace("{{header}}", "No Topics Available");
            email_html = email_html.replace("{{content}}", "<p>No content to display.</p>");
        }

        email_html
    }
}
