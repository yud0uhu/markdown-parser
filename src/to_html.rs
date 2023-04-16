use crate::{ast::AstNode, token::HeadingLevel};

pub fn generate_html(ast: &[AstNode]) -> String {
    let mut result = String::new();
    let mut is_in_list = false;
    for node in ast {
        match node {
            AstNode::Heading(HeadingLevel::H1, text) => {
                result.push_str(&format!("<h1>{}</h1>", text));
            }
            AstNode::Heading(HeadingLevel::H2, text) => {
                result.push_str(&format!("<h2>{}</h2>", text));
            }
            AstNode::Heading(HeadingLevel::H3, text) => {
                result.push_str(&format!("<h3>{}</h3>", text));
            }
            AstNode::Heading(HeadingLevel::H4, text) => {
                result.push_str(&format!("<h4>{}</h4>", text));
            }
            AstNode::Heading(HeadingLevel::H5, text) => {
                result.push_str(&format!("<h5>{}</h5>", text));
            }
            AstNode::Heading(HeadingLevel::H6, text) => {
                result.push_str(&format!("<h6>{}</h6>", text));
            }
            AstNode::BlockQuotes(text) => {
                result.push_str(&format!("<blockquote>{}</blockquote>", text));
            }
            AstNode::Lists(text) => {
                if !is_in_list {
                    result.push_str("<ul>");
                    is_in_list = true;
                }
                result.push_str(&format!("<li>{}</li>", text));
            }
            AstNode::Bold(text) => {
                result.push_str(&format!("<b>{}</b>", text));
            }
            AstNode::Italic(text) => {
                result.push_str(&format!("<i>{}</i>", text));
            }
            AstNode::Text(text) => {
                result.push_str(&text);
            }

            AstNode::Paragraph(nodes) => {
                result.push_str("<p>");
                result.push_str(&generate_html(nodes));
                result.push_str("</p>");
            }
        }
        // リストの終了タグはリスト全体の生成が終わった後に追加する
        if is_in_list {
            result.push_str("</ul>");
            is_in_list = false;
        }
    }
    result
}
