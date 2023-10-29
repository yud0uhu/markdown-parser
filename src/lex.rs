use crate::{
    ast::AstNode,
    lex, parse,
    token::{HeadingLevel, Token},
};
use regex::Regex;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut in_bold = false;
    let mut in_italic = false;

    for line in input.lines() {
        let mut chars = line.chars().peekable();

        if let Some(note_cap) = Regex::new(r"\[\^(.+)\]:\s(.+)").unwrap().captures(line) {
            let reference_name = note_cap[1].to_string();
            let note_content = note_cap[2].to_string();
            tokens.push(Token::NoteDefinition(reference_name, note_content));
            continue;
        }

        while let Some(c) = chars.next() {
            match (c, in_bold, in_italic) {
                ('#', false, false) => {
                    // Heading
                    let mut level = 1;
                    while chars.peek() == Some(&'#') {
                        chars.next();
                        level += 1;
                    }
                    // Skip whitespace
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }
                    tokens.push(Token::Heading(
                        match level {
                            1 => HeadingLevel::H1,
                            2 => HeadingLevel::H2,
                            3 => HeadingLevel::H3,
                            4 => HeadingLevel::H4,
                            5 => HeadingLevel::H5,
                            _ => HeadingLevel::H6,
                        },
                        chars.collect(),
                    ));
                    break;
                }

                ('>', false, false) => {
                    // BlockQuotes
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.trim().to_string()));
                        buffer.clear();
                    }
                    // Skip whitespace
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }
                    tokens.push(Token::BlockQuotes(chars.collect()));
                    break;
                }
                ('-', false, false) | ('+', false, false) => {
                    // Lists
                    if !buffer.trim().is_empty() {
                        tokens.push(Token::Text(buffer.trim().to_string()));
                        buffer.clear();
                    }
                    // Skip whitespace
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }
                    let mut list_buffer = String::new();
                    loop {
                        match chars.next() {
                            Some('\n') | None => {
                                if !list_buffer.trim().is_empty() {
                                    tokens.push(Token::Lists(list_buffer.trim().to_string()));
                                }
                                list_buffer.clear();
                                break;
                            }
                            Some(c) => {
                                list_buffer.push(c);
                            }
                        }
                    }
                }
                // '*' の前に '*' がある場合にのみ Token::Bold にマッチさせる
                // in_bold が true の場合、次の文字が '*' の場合にのみ Token::Bold を終了する
                ('*', false, false) if chars.peek() == Some(&'*') => {
                    // Bold
                    chars.next();
                    if !buffer.trim().is_empty() {
                        tokens.push(Token::Text(buffer.trim().to_string()));
                        buffer.clear();
                    }
                    in_bold = true;
                }

                ('*', true, false) if chars.peek() == Some(&'*') => {
                    // End of Bold
                    chars.next();
                    tokens.push(Token::Bold(buffer.trim().to_string()));
                    buffer.clear();
                    in_bold = false;
                }

                ('_', false, false) => {
                    // Italic
                    chars.next();
                    if !buffer.trim().is_empty() {
                        tokens.push(Token::Text(buffer.trim().to_string()));
                        buffer.clear();
                    }
                    in_italic = true;
                }

                ('_', false, true) if chars.peek() == Some(&'_') => {
                    // End of Italic
                    chars.next();
                    tokens.push(Token::Italic(buffer.trim().to_string()));
                    buffer.clear();
                    in_italic = false;
                }

                (_, false, false) => {
                    buffer.push(c);
                }

                _ => {
                    // Text
                    buffer.push(c);
                }
            }
        }
        if !buffer.trim().is_empty() {
            tokens.push(Token::Text(buffer.trim().to_string()));
            buffer.clear();
        }
    }
    tokens
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "\
        ## Heading 2\n\n> This is a blockquote.\n\nMore **bold** and __italic__ text.\n
        - List1\n
        - List2\n
        ### 注釈[^2]\n
        [^2]: 本文の語句や文章をとりあげてその意味を解説すること";

        let expected_output = vec![
            Token::Heading(HeadingLevel::H2, "Heading 2".to_string()),
            Token::BlockQuotes("This is a blockquote.".to_string()),
            Token::Text("More".to_string()),
            Token::Bold("bold".to_string()),
            Token::Text("and".to_string()),
            Token::Italic("italic".to_string()),
            Token::Text("text.".to_string()),
            Token::Lists("List1".to_string()),
            Token::Lists("List2".to_string()),
            Token::Heading(HeadingLevel::H3, "注釈[^2]".to_string()),
            Token::NoteDefinition(
                "2".to_string(),
                "本文の語句や文章をとりあげてその意味を解説すること".to_string(),
            ),
        ];

        assert_eq!(lex(input), expected_output);
    }
}
