use wasm_bindgen::prelude::*;

use pulldown_cmark::{html, Options, Parser};

#[wasm_bindgen]
pub fn markdown(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
