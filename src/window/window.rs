use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use compositing::compositor_task::{CompositorProxy, CompositorReceiver};
use compositing::windowing::WindowMethods;

use msg::constellation_msg;
use msg::constellation_msg::Key;

use net_traits::net_error_list::NetError;

use sutil::geometry::ScreenPx;
use layers::geometry::DevicePixel;
use layers::platform::surface::NativeDisplay;
use x11::xlib::Display;

use url::Url;
use sutil::cursor::Cursor;

use glutin;
use glutin::{Api, GlRequest};
use gleam::gl;

use euclid::scale_factor::ScaleFactor;
use euclid::size::{Size2D, TypedSize2D};

pub struct Window {
	window: glutin::Window,
}

impl Window {
	pub fn new() -> Window {
		let window = glutin::WindowBuilder::new()
			.with_title("miserve".to_owned())
			.with_dimensions(800, 600)
			.with_gl(GlRequest::Specific(Api::OpenGl, (2, 1)))
			.build()
			.unwrap();

		unsafe {
			window.make_current().expect("failed to make the context current");
		}

		gl::load_with(|s| window.get_proc_address(s));

		gl::clear_color(0.6, 0.6, 0.6, 1.0);
		gl::clear(gl::COLOR_BUFFER_BIT);
		gl::finish();

		window.swap_buffers().unwrap();

		Window {
			window: window,
		}
	}
}

impl WindowMethods for Window {
	fn create_compositor_channel(window: &Option<Rc<Window>>) -> (Box<CompositorProxy + Send>, Box<CompositorReceiver>) {
		super::Proxy::new(window)
	}

	fn supports_clipboard(&self) -> bool {
		true
	}

	fn hidpi_factor(&self) -> ScaleFactor<ScreenPx, DevicePixel, f32> {
		ScaleFactor::new(self.window.hidpi_factor())
	}

	fn framebuffer_size(&self) -> TypedSize2D<DevicePixel, u32> {
		let factor          = self.window.hidpi_factor() as u32;
		let (width, height) = self.window.get_inner_size().unwrap();

		Size2D::typed(width * factor, height * factor)
	}

	fn size(&self) -> TypedSize2D<ScreenPx, f32> {
		let (width, height) = self.window.get_inner_size().unwrap();

		Size2D::typed(width as f32, height as f32)
	}

	fn native_display(&self) -> NativeDisplay {
		unsafe {
			NativeDisplay::new(self.window.platform_display() as *mut Display)
		}
	}

	fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
		true
	}

	fn present(&self) {
		self.window.swap_buffers().unwrap();
	}

	fn set_page_title(&self, _: Option<String>) { }
	fn set_page_url(&self, _: Url) { }
	fn set_favicon(&self, _: Url) { }
	fn set_cursor(&self, _: Cursor) { }

	fn status(&self, _: Option<String>) { }

	fn load_start(&self, _: bool, _: bool) { }
	fn load_end(&self, _: bool, _: bool) { }
	fn load_error(&self, _: NetError, _: String) { }

	fn head_parsed(&self) { }

	fn handle_key(&self, _: Key, _: constellation_msg::KeyModifiers) { }
}

impl Deref for Window {
	type Target = glutin::Window;

	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

impl DerefMut for Window {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.window
	}
}
