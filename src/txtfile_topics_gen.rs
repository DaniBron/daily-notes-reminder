use crate::common;

use common::topics_generator::TopicsGenerator;
use common::topics_generator::Topics;
use common::topics_generator::BulletNode;

use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct TxtFileTopicsGenerator;

impl TopicsGenerator for TxtFileTopicsGenerator 
{
    fn generate_topics(&self, file_path: &str) -> Result<Topics, Box<dyn std::error::Error>>
    {
        // Attempt to open the Word document
        let doc = File::open(file_path)?;

        let mut topics: Topics = Topics::new();

        self.arrange_text_to_topics(&doc, &mut topics);

        if topics.is_empty() {
            return Err("No topics found in the document".into());
        }

        Ok(topics)
    }
}

impl TxtFileTopicsGenerator
{
    pub fn new() -> TxtFileTopicsGenerator
    {
        TxtFileTopicsGenerator { }
    }

    fn arrange_text_to_topics(&self, text_file: &File, topics: &mut Topics) {
        
        let reader = BufReader::new(text_file);

        let mut current_topic = String::new();

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    println!("Error reading line: {}", e.to_string());
                    continue;
                }
            };

            if line.starts_with("- ") {
                // New topic
                current_topic = line[2..].trim().to_string();
                topics.add(BulletNode {
                    content: current_topic.clone(),
                    children: Vec::new(),
                    level: 0,
                });
            } 
            else if line.starts_with("o ") {
                // Bullet point under the current topic
                let last_bullet = BulletNode {
                    content: line[2..].trim().to_string(),
                    children: Vec::new(),
                    level: 1,
                };

                let last_topic = topics.root_nodes.last_mut().unwrap();

                last_topic.children.push(last_bullet);
            } 
            else if line.starts_with("* ") {
                // Sub-bullet under the last bullet
                let sub_bullet = BulletNode {
                    content: line[2..].trim().to_string(),
                    children: Vec::new(),
                    level: 2,
                };

                let last_bullet = topics.last_child().unwrap();
                
                last_bullet.children.push(sub_bullet);}
        }
    }
}