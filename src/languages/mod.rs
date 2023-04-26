use std::error::Error;

mod rust;

pub trait LanguageSupport {
    fn create_files<'a>(
        title_slug: &'a str,
        code_snippet: &'a str,
        example_testcase: &'a str,
    ) -> Result<&'a str, Box<dyn Error>>;
}

pub fn create_files<'a>(
    title_slug: &'a str,
    code_snippet: &'a str,
    example_testcase: &'a str,
) -> Result<&'a str, Box<dyn Error>> {
    rust::Rust::create_files(title_slug, code_snippet, example_testcase)
}
