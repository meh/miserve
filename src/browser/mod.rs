mod browser;
pub use self::browser::Browser;

mod tab;
pub use self::tab::Tab;

mod window;
pub use self::window::Window;

pub mod buffer;
pub use self::buffer::Buffer;

use std::rc::Rc;
use std::cell::RefCell;
use glutin;

#[derive(Debug)]
pub enum Event {
	Idle,
	Window(glutin::Event),
	Buffer(Rc<RefCell<Buffer>>, buffer::Event),
}
