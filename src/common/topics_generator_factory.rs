// factory.rs

use crate::common::topics_generator::TopicsGenerator;
//use crate::txtfile_topics_gen::TxtFileTopicsGenerator;
use crate::tomlfile_topics_gen::TomlFileTopicsGenerator;

pub fn topics_gen_factory(file_path: &str) -> Option<impl TopicsGenerator> {
    // if file_path.ends_with(".txt") {
    //     Some(TxtFileTopicsGenerator)
    // } 
    if file_path.ends_with(".toml") {
        Some(TomlFileTopicsGenerator)
    }
    
    else {
        None
    }
}