use sabani_core::net::HttpClient;
use sabani_core::renderer::html::{parser::HtmlParser, token::HtmlTokenizer};
use sabani_core::utils::convert_dom_to_string;

fn main() {
  let client = HttpClient::new();
  let response = client
    .get("localhost".to_string(), 8000, "/test.html".to_string())
    .unwrap();
  let html = response.body();
  let t = HtmlTokenizer::new(html);
  let window = HtmlParser::new(t).construct_tree();
  let document = window.borrow().document();
  let document_html_string = convert_dom_to_string(&Some(document));
  println!("{document_html_string}");
}
