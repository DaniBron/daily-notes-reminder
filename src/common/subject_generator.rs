use crate::common::topics_generator::BlogPosts;

pub trait SubjectGenerator {
    fn generate_subject(&self, topics: &BlogPosts) -> Result<String, Box<dyn std::error::Error>>;
}