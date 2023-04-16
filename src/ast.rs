use crate::token::HeadingLevel;

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Heading(HeadingLevel, String),
    Bold(String),
    Italic(String),
    Text(String),
    BlockQuotes(String),
    Lists(String),
    Paragraph(Vec<AstNode>),
}
