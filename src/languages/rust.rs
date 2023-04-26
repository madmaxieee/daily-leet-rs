use std::error::Error;

use crate::languages::LanguageSupport;

pub struct Rust {}

impl LanguageSupport for Rust {
    fn create_files<'a>(
        title_slug: &'a str,
        code_snippet: &'a str,
        example_testcase: &'a str,
    ) -> Result<&'a str, Box<dyn Error>> {
        println!("Creating files for Rust");
        println!("title_slug: {}", title_slug);
        println!("code_snippet: {}", code_snippet);
        println!("example_testcase: {}", example_testcase);
        Ok("Rust")
    }
}
