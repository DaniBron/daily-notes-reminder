use crate::common;

use common::topics_generator::TopicsGenerator;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};




pub struct TxtFileTopicsGenerator;

impl TopicsGenerator for TxtFileTopicsGenerator 
{
    fn generate_topics(&self, file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>>
    {
        // Attempt to open the Word document
        let doc = File::open(file_path)?;

        let mut topics: HashMap<String, Vec<String>> = HashMap::new();

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


    fn arrange_text_to_topics(&self, text_file: &File, topics: &mut HashMap<String, Vec<String>>) {

        let reader = BufReader::new(text_file);

        let mut current_topic = String::new();

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    println!("Error reading line: {}", e.to_string());
                    "".to_string()  // Return an empty string
                }
            };

            if line.starts_with("- ") {
                // New topic starts here
                current_topic = line[2..].trim().to_string();
                topics.entry(current_topic.clone()).or_insert_with(Vec::new);

                //println!("New topic: {}", current_topic);
            } 
            
            else if line.starts_with("o ") {
                // Bullet point for the current topic
                if let Some(bullets) = topics.get_mut(&current_topic) {
                    bullets.push(line[2..].trim().to_string());

                    //println!("New bullet point: {}", line[2..].trim());
                }
            }
        }
    }

}