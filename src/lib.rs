extern crate cfg_if;
extern crate wasm_bindgen;

mod ast;
mod lex;
mod parse;
mod to_html;
mod token;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn text_to_token(input_text: &str) -> String {
    let tokens = lex::lex(input_text);
    let ast = parse::parse(&tokens);
    to_html::generate_html(&ast)
}
