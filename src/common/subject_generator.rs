use std::collections::HashMap;

pub trait SubjectGenerator {
    fn generate_subject(&self, topics: &HashMap<String, Vec<String>>) -> Result<String, Box<dyn std::error::Error>>;
}