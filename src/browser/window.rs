use std::rc::Rc;
use std::cell::RefCell;

use servo::gl;
use browser::{Buffer};

/// This struct holds the texture of the size of the window, where the buffer
/// it contains is rendered to.
pub struct Window {
	buffer: Option<Rc<RefCell<Buffer>>>,

	width:  u32,
	height: u32,
	
	x: u32,
	y: u32,
}

impl Window {
	pub fn new() -> Self {
		Window {
			buffer: None,

			width:  0,
			height: 0,

			x: 0,
			y: 0,
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.width  = width;
		self.height = height;
	}

	pub fn position(&mut self, x: u32, y: u32) {
		self.x = x;
		self.y = y;
	}

	pub fn assign(&mut self, buffer: Rc<RefCell<Buffer>>) {
		self.buffer = Some(buffer);
	}

	pub fn buffer(&self) -> Option<Rc<RefCell<Buffer>>> {
		self.buffer.clone()
	}

	pub fn render(&mut self) {
		if let Some(buffer) = self.buffer.as_ref() {
			buffer.borrow_mut().viewport((self.x, self.y), (self.width, self.height));
			buffer.borrow_mut().render();
		}
		else {
			gl::clear_color(0.6, 0.6, 0.6, 1.0);
			gl::clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn x(&self) -> u32 {
		self.x
	}

	pub fn y(&self) -> u32 {
		self.y
	}
}
