use exports::dashbook_markdown;
use pulldown_cmark::{html, Options, Parser};

wit_bindgen::generate!({
    world: "dashbook-markdown",
});

export!(Component);
struct Component;

impl dashbook_markdown::Guest for Component {
    fn markdown(input: String) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&input, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
