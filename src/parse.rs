use crate::{
    ast::AstNode,
    lex, parse,
    token::{HeadingLevel, Token},
};

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
    let mut result = Vec::new();
    let mut current_paragraph = Vec::new();
    let mut in_bold = false;
    let mut in_italic = false;

    for token in tokens {
        match token {
            Token::Heading(level, text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::Heading(level.clone(), text.clone()));
            }
            Token::BlockQuotes(text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::BlockQuotes(text.clone()));
            }
            Token::Lists(text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::Lists(text.clone()));
            }
            Token::Bold(text) => {
                if !in_bold {
                    current_paragraph.push(AstNode::Bold(text.clone()));
                    in_bold = true;
                } else {
                    current_paragraph.push(AstNode::Text(text.clone()));
                    in_bold = false;
                }
            }
            Token::Italic(text) => {
                if !in_italic {
                    current_paragraph.push(AstNode::Italic(text.clone()));
                    in_italic = true;
                } else {
                    current_paragraph.push(AstNode::Text(text.clone()));
                    in_italic = false;
                }
            }
            Token::Text(text) => {
                if !in_bold && !in_italic {
                    current_paragraph.push(AstNode::Text(text.clone()));
                } else {
                    let mut inner_paragraph = Vec::new();
                    inner_paragraph.push(AstNode::Text(text.clone()));
                    if in_bold {
                        inner_paragraph.push(AstNode::Bold(text.clone()));
                    }
                    if in_italic {
                        inner_paragraph.push(AstNode::Italic(text.clone()));
                    }
                    in_bold = false;
                    in_italic = false;
                }
            }
        }
    }

    if !current_paragraph.is_empty() {
        result.push(AstNode::Paragraph(current_paragraph.clone()));
        current_paragraph.clear();
    }

    result
}
#[test]
fn test_lex_and_parse() {
    let input = "\
# Hello, world!\n
> This is a blockquote\n
This is a **markdown** __parser__.";
    let expected_output = vec![
        AstNode::Heading(HeadingLevel::H1, "Hello, world!".to_string()),
        AstNode::BlockQuotes("This is a blockquote".to_string()),
        AstNode::Paragraph(vec![
            AstNode::Text("This is a ".to_string()),
            AstNode::Bold("markdown".to_string()),
            AstNode::Italic("parser".to_string()),
        ]),
    ];
    let tokens = lex::lex(input);
    let output = parse::parse(&tokens);
    assert_eq!(output, expected_output);
}
