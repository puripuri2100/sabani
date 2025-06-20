use crate::renderer::dom::node::Node;

#[cfg(feature = "wasabi")]
use alloc::format;
#[cfg(feature = "wasabi")]
use alloc::rc::Rc;
#[cfg(feature = "wasabi")]
use alloc::string::String;
#[cfg(feature = "wasabi")]
use core::cell::RefCell;

#[cfg(not(feature = "wasabi"))]
use std::cell::RefCell;
#[cfg(not(feature = "wasabi"))]
use std::format;
#[cfg(not(feature = "wasabi"))]
use std::rc::Rc;

pub fn convert_dom_to_string(root: &Option<Rc<RefCell<Node>>>) -> String {
  let mut result = String::from("\n");
  convert_dom_to_string_internal(root, 0, &mut result);

  result
}

fn convert_dom_to_string_internal(
  node: &Option<Rc<RefCell<Node>>>,
  depth: usize,
  result: &mut String,
) {
  if let Some(n) = node {
    result.push_str(&"  ".repeat(depth));
    result.push_str(&format!("{:?}", n.borrow().kind()));
    result.push('\n');

    convert_dom_to_string_internal(&n.borrow().first_child(), depth + 1, result);
    convert_dom_to_string_internal(&n.borrow().next_sibling(), depth, result);
  }
}
