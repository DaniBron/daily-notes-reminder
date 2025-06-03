use rand::Rng;
use std::error::Error;
use crate::common::subject_generator::SubjectGenerator;
use crate::common::topics_generator::{BlogPosts, Post}; // Import your structs

pub struct RandomSubjectGenerator {}

impl SubjectGenerator for RandomSubjectGenerator {
    // The trait method's return type should be Result<String, Box<dyn Error>> as HTML is a String
    fn generate_subject(&self, blog_posts: &BlogPosts) -> Result<String, Box<dyn Error>> {
        // We'll generate HTML for a single random post
        match self.generate_html_for_random_post(blog_posts) {
            Some(html_string) => Ok(html_string),
            None => Err("No posts available to generate a subject from.".into()),
        }
    }
}

impl RandomSubjectGenerator {
    pub fn new() -> RandomSubjectGenerator {
        RandomSubjectGenerator {}
    }

    // This function now returns a random `&Post` instead of `&BulletNode`
    fn get_random_post<'a>(&'a self, blog_posts: &'a BlogPosts) -> Option<&Post> {
        if blog_posts.post.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..blog_posts.post.len());
        
        // Return a reference to the randomly selected post
        blog_posts.post.get(random_index)
    }

    // This function now takes `&BlogPosts` and generates HTML for one random post.
    // It is no longer `generate_html_str` but `generate_html_for_random_post` for clarity.
    fn generate_html_for_random_post(&self, blog_posts: &BlogPosts) -> Option<String> {
        // Load the HTML template
        let mut email_html_template: String = include_str!("styles/style.html").to_string();

        if let Some(post) = self.get_random_post(blog_posts) {
            // Replace post title
            email_html_template = email_html_template.replace("{{post_title}}", &post.title);

            // Replace tags
            let tags_html = post.tags.iter()
                .map(|tag| format!("<span class=\"tag\">{}</span>", tag))
                .collect::<Vec<String>>()
                .join("");
            email_html_template = email_html_template.replace("{{post_tags}}", &tags_html);

            // Generate content for the post body
            let mut post_body_content = String::new();
            
            for section in &post.sections {
                match section.section_type.as_str() {
                    "introduction" | "conclusion" => {
                        if let Some(content) = &section.content {
                            post_body_content.push_str(&format!("<p>{}</p>", content));
                        }
                    },
                    "heading_with_paragraphs" => {
                        if let Some(heading) = &section.heading {
                            post_body_content.push_str(&format!("<h2>{}</h2>", heading));
                        }
                        if let Some(paragraphs) = &section.paragraphs {
                            for p_text in paragraphs {
                                post_body_content.push_str(&format!("<p>{}</p>", p_text));
                            }
                        }
                    },
                    "heading_with_paragraphs_and_bullets" => {
                        if let Some(heading) = &section.heading {
                            post_body_content.push_str(&format!("<h2>{}</h2>", heading));
                        }
                        if let Some(paragraphs) = &section.paragraphs {
                            for p_text in paragraphs {
                                post_body_content.push_str(&format!("<p>{}</p>", p_text));
                            }
                        }
                        if let Some(bullets) = &section.bullets {
                            post_body_content.push_str("<ul>");
                            for bullet_text in bullets {
                                post_body_content.push_str(&format!("<li>{}</li>", bullet_text));
                            }
                            post_body_content.push_str("</ul>");
                        }
                        if let Some(paragraphs_after_bullets) = &section.paragraphs_after_bullets {
                            for p_text in paragraphs_after_bullets {
                                post_body_content.push_str(&format!("<p>{}</p>", p_text));
                            }
                        }
                    },
                    "heading_with_paragraphs_and_lists" => {
                        // This handles sections like "Relational Databases (SQL)" with "Key Features", "Use Cases", "Examples"
                        if let Some(heading) = &section.heading {
                            post_body_content.push_str(&format!("<h2>{}</h2>", heading));
                        }
                        if let Some(paragraphs) = &section.paragraphs {
                            for p_text in paragraphs {
                                post_body_content.push_str(&format!("<p>{}</p>", p_text));
                            }
                        }
                        if let Some(sub_sections) = &section.sub_sections {
                            for sub_sec in sub_sections {
                                // Sub-section title (e.g., "Key Features") as h3 or strong
                                post_body_content.push_str(&format!("<h3>{}</h3>", sub_sec.title));
                                post_body_content.push_str("<ul>");
                                for item in &sub_sec.items {
                                    post_body_content.push_str(&format!("<li>{}</li>", item));
                                }
                                post_body_content.push_str("</ul>");
                            }
                        }
                    },
                    // Add more cases for other section types if they appear in your TOML
                    _ => {
                        // Handle unknown section types, or log a warning
                        eprintln!("WARNING: Unknown section type encountered: {}", section.section_type);
                    }
                }
            }

            // Replace the main content placeholder
            email_html_template = email_html_template.replace("{{post_body}}", &post_body_content);
            Some(email_html_template)

        } else {
            // No posts found, return None
            None
        }
    }
}