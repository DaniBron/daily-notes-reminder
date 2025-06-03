use crate::common;
use common::topics_generator::TopicsGenerator;
use std::fs::File;
use toml;
use std::io::Read; // Add this line
use common::topics_generator::BlogPosts;

pub struct TomlFileTopicsGenerator;

impl TopicsGenerator for TomlFileTopicsGenerator {
    fn generate_topics(&self, file_path: &str) -> Result<BlogPosts, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the entire TOML into our new BlogPosts struct
        let blog_posts_data: BlogPosts = toml::from_str(&contents)?;

        if blog_posts_data.post.is_empty() {
            return Err("No posts found in the document".into()); // Adjust error message
        }

        Ok(blog_posts_data) // Return the Vec<Post>
    }
}