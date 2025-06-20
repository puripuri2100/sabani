use crate::renderer::page::Page;

#[cfg(feature = "wasabi")]
use alloc::{rc::Rc, vec::Vec};
#[cfg(feature = "wasabi")]
use core::cell::RefCell;

#[cfg(not(feature = "wasabi"))]
use std::cell::RefCell;
#[cfg(not(feature = "wasabi"))]
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Browser {
  active_page_index: usize,
  pages: Vec<Rc<RefCell<Page>>>,
}

impl Browser {
  pub fn new() -> Rc<RefCell<Self>> {
    let mut page = Page::new();

    let browser = Rc::new(RefCell::new(Self {
      active_page_index: 0,
      pages: Vec::new(),
    }));

    page.set_browser(Rc::downgrade(&browser));
    browser.borrow_mut().pages.push(Rc::new(RefCell::new(page)));

    browser
  }

  pub fn current_page(&self) -> Rc<RefCell<Page>> {
    self.pages[self.active_page_index].clone()
  }
}
