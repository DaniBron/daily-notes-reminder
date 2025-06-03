// use crate::common;

// use common::topics_generator::TopicsGenerator;
// use common::topics_generator::Topics;
// use common::topics_generator::BulletNode;

// use std::fs::File;
// use std::io::{BufRead, BufReader};
// pub struct TxtFileTopicsGenerator;

// impl TopicsGenerator for TxtFileTopicsGenerator 
// {
//     fn generate_topics(&self, file_path: &str) -> Result<Topics, Box<dyn std::error::Error>>
//     {
//         // Attempt to open the Word document
//         let doc = File::open(file_path)?;

//         let mut topics: Topics = Topics::new();

//         self.arrange_text_to_topics(&doc, &mut topics);

//         if topics.is_empty() {
//             return Err("No topics found in the document".into());
//         }

//         Ok(topics)
//     }
// }

// impl TxtFileTopicsGenerator
// {
//     pub fn new() -> TxtFileTopicsGenerator
//     {
//         TxtFileTopicsGenerator { }
//     }

//     fn arrange_text_to_topics(&self, text_file: &File, topics: &mut Topics) {
//     let reader = BufReader::new(text_file);
//     let mut current_topic: Option<String> = None;
//     let mut current_subtopic: Option<usize> = None; // Index of last subtopic in children

//     for line_result in reader.lines() {
//         let line = match line_result {
//             Ok(line) => line,
//             Err(e) => {
//                 println!("Error reading line: {}", e);
//                 continue;
//             }
//         };

//         if line.starts_with("- ") {
//             current_topic = Some(line[2..].trim().to_string());
//             current_subtopic = None;
//             topics.add(BulletNode {
//                 content: current_topic.clone().unwrap(),
//                 children: Vec::new(),
//                 level: 0,
//             });
//         } else if line.starts_with("**") {
//             current_subtopic = Some(topics.root_nodes.last_mut().unwrap().children.len());
//             let content = &line[2..line.len() - 3].trim();
//             let last_bullet = BulletNode {
//                 content: content.to_string(),
//                 children: Vec::new(),
//                 level: 1,
//             };
//             if let Some(last_topic) = topics.root_nodes.last_mut() {
//                 last_topic.children.push(last_bullet);
//             }
//             println!("Sub topic: {}", line[2..].trim());
//         } else if line.starts_with("o ") && current_subtopic.is_none() {
//             // Only trim "o " for root-level bullets
//             let last_bullet = BulletNode {
//                 content: line[2..].trim().to_string(),
//                 children: Vec::new(),
//                 level: 1,
//             };
//             if let Some(last_topic) = topics.root_nodes.last_mut() {
//                 last_topic.children.push(last_bullet);
//             }
//         } else if line.starts_with("o ") || !line.trim().is_empty() {
//             // Preserve full line under subtopic
//             let content = if line.starts_with("o ") { line[2..].trim().to_string() } else { line.trim().to_string() };
//             let last_bullet = BulletNode {
//                 content,
//                 children: Vec::new(),
//                 level: current_subtopic.is_some().then(|| 2).unwrap_or(1),
//             };
//             if let Some(last_topic) = topics.root_nodes.last_mut() {
//                 if let Some(sub_idx) = current_subtopic {
//                     if sub_idx < last_topic.children.len() {
//                         last_topic.children[sub_idx].children.push(last_bullet);
//                     } else {
//                         last_topic.children.push(last_bullet); // Fallback
//                     }
//                 } else {
//                     last_topic.children.push(last_bullet);
//                 }
//             }
//         } else if line.starts_with("* ") {
//             let sub_bullet = BulletNode {
//                 content: line[2..].trim().to_string(),
//                 children: Vec::new(),
//                 level: 3,
//             };
//             if let Some(last_topic) = topics.root_nodes.last_mut() {
//                 if let Some(sub_idx) = current_subtopic {
//                     if sub_idx < last_topic.children.len() {
//                         last_topic.children[sub_idx].children.push(sub_bullet);
//                     }
//                 }
//             }
//         }
//     }
// }
// }