use std::collections::HashMap;

pub trait TopicsGenerator {
    fn generate_topics(&self, file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>>;
}