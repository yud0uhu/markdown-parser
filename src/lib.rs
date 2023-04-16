extern crate cfg_if;
extern crate wasm_bindgen;

// use pulldown_cmark::{html, Options, Parser};
// use cfg_if::cfg_if;

mod ast;
mod lex;
mod parse;
mod to_html;
mod token;

use wasm_bindgen::prelude::*;

// cfg_if! {
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello,{}!", name));
// }

// #[wasm_bindgen]
// pub fn pulldown_cmark(source_text: &str) -> String {
//     let markdown_input = source_text;

//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     let parser = Parser::new_ext(markdown_input, options);

//     let mut html_output = String::new();
//     html::push_html(&mut html_output, parser);
//     html_output
// }

#[wasm_bindgen]
pub fn text_to_token(input_text: &str) -> String {
    let tokens = lex::lex(input_text);
    let ast = parse::parse(&tokens);
    to_html::generate_html(&ast)
}
