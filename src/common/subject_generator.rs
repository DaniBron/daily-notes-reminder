use crate::common::topics_generator::Topics;

pub trait SubjectGenerator {
    fn generate_subject(&self, topics: &Topics) -> Result<String, Box<dyn std::error::Error>>;
}