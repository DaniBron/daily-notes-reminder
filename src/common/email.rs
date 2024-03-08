pub trait Email {
    fn send_email(&self, subject: &str, text: String) -> Result<(), Box<dyn std::error::Error>>;
}