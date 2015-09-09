use std::rc::Rc;
use std::cell::RefCell;

use browser::{Browser, Window};

pub struct Tab {
	windows: Vec<Rc<RefCell<Window>>>,
}

impl Tab {
	pub fn new(browser: &Browser) -> Self {
		Tab {
			windows: Vec::new(),
		}
	}

	pub fn open(&mut self) -> Rc<RefCell<Window>> {
		let window = Rc::new(RefCell::new(Window::new()));
		self.windows.push(window.clone());

		window
	}

	pub fn render(&mut self) {
		for window in &self.windows {
			window.borrow_mut().render();
		}
	}
}
