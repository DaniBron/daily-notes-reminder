use serde::Deserialize;

#[derive(Debug)]
pub struct Topics {
    pub root_nodes: Vec<BulletNode>,
}

impl Topics {
    // Helper method to create a new empty Topics
    pub fn new() -> Self {
        Topics { root_nodes: Vec::new() }
    }
    
    // Add getter for root_nodes
    pub fn root_nodes(&self) -> &[BulletNode] {
        &self.root_nodes
    }

    pub fn add(&mut self, node: BulletNode) {
        self.root_nodes.push(node);
    }

    pub fn is_empty(&self) -> bool {
        self.root_nodes.is_empty()
    }

    pub fn last_child(&mut self) -> Option<&mut BulletNode> {
        if self.root_nodes.is_empty() {
            return None;
        }
        
        let last_root = self.root_nodes.last_mut()?;
        
        if last_root.children.is_empty() {
            return Some(last_root);
        }
        
        Some(last_root.children.last_mut()?)
    }
}

#[derive(Debug)]
pub struct BulletNode {
    pub content: String,
    pub children: Vec<BulletNode>,
    pub level: usize,
}

impl BulletNode {
    // Helper method to create a new node
    pub fn new(content: String, level: usize) -> Self {
        BulletNode {
            content,
            children: Vec::new(),
            level,
        }
    }
    
    // Getters
    pub fn content(&self) -> &str {
        &self.content
    }
    
    pub fn children(&self) -> &[BulletNode] {
        &self.children
    }
    
    pub fn level(&self) -> usize {
        self.level
    }
}

// --- Main Struct for the Entire TOML File ---
// This will hold a vector of Post structs because [[post]] indicates an array of posts.
#[derive(Debug, Deserialize)]
pub struct BlogPosts {
    pub post: Vec<Post>,
}

// --- Post Struct ---
// Represents a single blog post.
#[derive(Debug, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    // A post contains an array of sections
    pub sections: Vec<Section>,
}

// --- Section Struct ---
// Represents a generic section within a post.
// The fields are made `Option<T>` because not all section types will have all fields.
#[derive(Debug, Deserialize)]
pub struct Section {
    #[serde(rename = "type")] // Map 'type' from TOML to 'section_type' to avoid Rust keyword clash
    pub section_type: String,
    pub content: Option<String>,
    pub heading: Option<String>,
    pub paragraphs: Option<Vec<String>>,
    pub bullets: Option<Vec<String>>,
    pub paragraphs_after_bullets: Option<Vec<String>>,
    pub parent_topic: Option<String>,
    // Nested `sub_sections` for "heading_with_paragraphs_and_lists" type
    pub sub_sections: Option<Vec<SubSection>>,
}

// --- SubSection Struct ---
// Represents a sub-section with a title and a list of items (e.g., Key Features, Use Cases).
#[derive(Debug, Deserialize)]
pub struct SubSection {
    pub title: String,
    pub items: Vec<String>,
}

pub trait TopicsGenerator {
    fn generate_topics(&self, file_path: &str) -> Result<BlogPosts, Box<dyn std::error::Error>>;
}


