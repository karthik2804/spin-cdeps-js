#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::markdown_renderer::markdown_fns::Guest;
use pulldown_cmark::{html, Parser};

struct Component;

impl Guest for Component {
    fn render(content: String) -> String {
        let parser = Parser::new(&content);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}

bindings::export!(Component with_types_in bindings);
