#[derive(Debug)]
pub enum TokenType {
    HeadingOne {
        literal: String,
        start_tag: String,
        end_tag: String,
    },
    HeadingTwo {
        literal: String,
        start_tag: String,
        end_tag: String,
    },
    HeadingThree {
        literal: String,
        start_tag: String,
        end_tag: String,
    },
    Paragraph {
        literal: String,
        start_tag: String,
        end_tag: String,
    },
    Image {
        literal: String,
        start_tag: String,
        end_tag: String,
    },
    Code {
        literal: String,
        language: String,
        start_tag: String,
        end_tag: String,
    },
}
