use crate::browser::Browser;
use crate::http::HttpResponse;
use crate::renderer::dom::node::Window;
use crate::renderer::html::{parser::HtmlParser, token::HtmlTokenizer};
use crate::utils::convert_dom_to_string;

#[cfg(feature = "wasabi")]
use alloc::rc::{Rc, Weak};
#[cfg(feature = "wasabi")]
use alloc::string::String;
#[cfg(feature = "wasabi")]
use core::cell::RefCell;

#[cfg(not(feature = "wasabi"))]
use std::cell::RefCell;
#[cfg(not(feature = "wasabi"))]
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Page {
  browser: Weak<RefCell<Browser>>,
  frame: Option<Rc<RefCell<Window>>>,
}

impl Page {
  pub fn new() -> Self {
    Self {
      browser: Weak::new(),
      frame: None,
    }
  }

  pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
    self.browser = browser
  }

  pub fn receive_response(&mut self, response: HttpResponse) -> String {
    self.create_frame(response.body());

    // デバッグ用にDOMツリーを文字列化して返す
    if let Some(frame) = &self.frame {
      let dom = frame.borrow().document().clone();
      let debug = convert_dom_to_string(&Some(dom));
      return debug;
    }
    "".to_string()
  }

  fn create_frame(&mut self, html: String) {
    let html_tokenizer = HtmlTokenizer::new(html);
    let frame = HtmlParser::new(html_tokenizer).construct_tree();
    self.frame = Some(frame)
  }
}
