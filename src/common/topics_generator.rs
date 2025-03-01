//use std::collections::HashMap;

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



pub trait TopicsGenerator {
    fn generate_topics(&self, file_path: &str) -> Result<Topics, Box<dyn std::error::Error>>;
}


