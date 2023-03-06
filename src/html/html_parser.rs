use std::fs::{self, File};
use std::io::Write;

use super::lexer::Lexer;
use super::token::TokenType;

pub struct HtmlParser<'a> {
    lexer: Lexer<'a>,
    html_tags: Vec<String>,
}

impl HtmlParser<'_> {
    pub fn new(lexer: Lexer) -> HtmlParser {
        HtmlParser {
            lexer,
            html_tags: Vec::<String>::new(),
        }
    }

    pub fn generate_html(&mut self) {
        for tokens in &self.lexer.tokens {
            match tokens {
                TokenType::HeadingOne {
                    literal,
                    start_tag,
                    end_tag,
                }
                | TokenType::HeadingTwo {
                    literal,
                    start_tag,
                    end_tag,
                }
                | TokenType::HeadingThree {
                    literal,
                    start_tag,
                    end_tag,
                }
                | TokenType::Paragraph {
                    literal,
                    start_tag,
                    end_tag,
                } => {
                    let html_tag = Self::generate_html_tags(literal, start_tag, end_tag);
                    self.html_tags.push(html_tag);
                }
                TokenType::Code {
                    literal,
                    language,
                    start_tag,
                    end_tag,
                } => {
                    let html_tag = Self::generate_code_block(literal, start_tag, end_tag, language);
                    self.html_tags.push(html_tag)
                }
                _ => {}
            };
        }
    }

    pub fn generate_html_tags(literal: &String, start_tag: &String, end_tag: &String) -> String {
        format!("{}{}{}", start_tag, literal, end_tag)
    }

    pub fn generate_code_block(
        literal: &String,
        start_tag: &String,
        end_tag: &String,
        language: &String,
    ) -> String {
        format!("{}{}\">{}{}", start_tag, language, literal, end_tag)
    }

    pub fn generte_html_page(&mut self) {
        let filename = format!("./static/{}.html", self.lexer.title.replace(" ", ""));
        let mut file = File::create(filename).unwrap();
        // Load in template file
        let template_blog_html = fs::read_to_string("./templates/blog_post_template.html").unwrap();
        // Change title in template text
        let html_with_title = template_blog_html.replace("{TITLE_OF_DOC}", &self.lexer.title);
        // Add the html elements to the text
        let joined = self.html_tags.join("\n");
        let html_with_elements_and_title = html_with_title.replace("{BLOG_POST}", joined.as_str());

        // Write html to file
        file.write_all(html_with_elements_and_title.as_bytes());
    }
}
