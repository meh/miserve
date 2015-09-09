use std::cell::{Cell, RefCell};
use std::sync::mpsc::{channel, Sender};
use std::rc::Rc;
use std::fmt;

use servo::compositing::CompositorEventListener;
use servo::compositing::windowing::{WindowEvent, WindowMethods};
use servo::msg::constellation_msg::{Key, KeyModifiers};
use servo::net_traits::net_error_list::NetError;
use servo::url::Url;
use servo::euclid::size::{Size2D, TypedSize2D};
use servo::euclid::point::Point2D;
use servo::layers::geometry::DevicePixel;
use servo::util::geometry::ScreenPx;
use servo::euclid::scale_factor::ScaleFactor;
use servo::util::cursor::Cursor;

use servo::compositing::Constellation;
use servo::compositing::CompositorTask;
use servo::layout::layout_task::LayoutTask;
use servo::script::script_task::ScriptTask;

use servo::compositing::compositor_task::{CompositorProxy, CompositorReceiver, Msg};
use servo::layers::platform::surface::NativeDisplay;
use glutin;
use x11::xlib::Display;

use browser::{self, Browser};

/// This struct holds the compositor for a page.
pub struct Buffer {
	window:     Rc<Window>,
	compositor: Box<CompositorEventListener + 'static>,

	width:  u32,
	height: u32,
}

impl Buffer {
	pub fn new(browser: &Browser) -> Rc<RefCell<Self>> {
		let window            = Rc::new(Window::new(browser));
		let (proxy, receiver) = WindowMethods::create_compositor_channel(&Some(window.clone()));

		let time_profiler = browser.profiler().time();
		let mem_profiler  = browser.profiler().mem();

		let constellation = Constellation::<LayoutTask, ScriptTask>::start(
			proxy.clone_compositor_proxy(),
			browser.resource(),
			browser.cache().image(), browser.cache().font(),
			time_profiler.clone(), mem_profiler.clone(), None,
			browser.storage(), true);

		let mut compositor = CompositorTask::create(Some(window.clone()), proxy, receiver,
			constellation, time_profiler, mem_profiler);

		compositor.handle_events(vec![WindowEvent::InitializeCompositing]);

		let buffer = Rc::new(RefCell::new(Buffer {
			window:     window.clone(),
			compositor: compositor,

			width:  0,
			height: 0,
		}));

		window.buffer(buffer.clone());

		buffer
	}

	pub fn idle(&mut self) {
		self.compositor.handle_events(vec![
			WindowEvent::Idle]);
	}

	pub fn render(&mut self) {
		self.compositor.handle_events(vec![
			WindowEvent::Refresh]);
	}

	pub fn viewport(&mut self, (x, y): (u32, u32), (width, height): (u32, u32)) {
		self.compositor.handle_events(vec![
			WindowEvent::Viewport(Point2D::typed(x, y), Size2D::typed(width, height))]);

		if self.width != width || self.height != height {
			self.window.resize(width, height);
			self.compositor.handle_events(vec![
				WindowEvent::Resize(Size2D::typed(width, height))]);
		}
	}

	pub fn go(&mut self, url: Url) {
		self.compositor.handle_events(vec![
			WindowEvent::LoadUrl(url.to_string())]);
	}

	pub fn title(&self) -> Option<String> {
		self.window.title.borrow().clone()
	}

	pub fn url(&self) -> Option<Url> {
		self.window.url.borrow().clone()
	}
}

impl fmt::Debug for Buffer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Buffer")
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		self.compositor.shutdown();
	}
}

struct Window {
	glutin: Rc<glutin::Window>,
	buffer: RefCell<Option<Rc<RefCell<Buffer>>>>,
	events: Rc<RefCell<Vec<browser::Event>>>,

	pub size:    Cell<(u32, u32)>,
	pub title:   RefCell<Option<String>>,
	pub url:     RefCell<Option<Url>>,
	pub favicon: RefCell<Option<Url>>,
	pub cursor:  RefCell<Option<Cursor>>,
	pub status:  RefCell<Option<String>>,
	pub load:    RefCell<Option<Load>>,
}

#[derive(Debug)]
pub enum Event {
	Present,
	Resize(Size2D<u32>),
	Position(Point2D<i32>),
	Title(Option<String>),
	Url(Url),
	Favicon(Url),
	Cursor(Cursor),
	Status(Option<String>),
	Load(Load),
	Head,
	Key { key: Key, modifier: KeyModifiers },
}

#[derive(Debug)]
pub enum Load {
	Start {
		back:    bool,
		forward: bool,
	},

	End {
		back:    bool,
		forward: bool,
	},

	Error {
		url: String,
		//code: NetError
	}
}

impl Window {
	pub fn new(browser: &Browser) -> Self {
		Window {
			glutin: browser.window(),
			buffer: RefCell::new(None),
			events: browser.events(),
			size:   Cell::new((0, 0)),

			title:   RefCell::new(None),
			url:     RefCell::new(None),
			favicon: RefCell::new(None),
			cursor:  RefCell::new(None),
			status:  RefCell::new(None),
			load:    RefCell::new(None),
		}
	}

	pub fn buffer(&self, buffer: Rc<RefCell<Buffer>>) {
		*self.buffer.borrow_mut() = Some(buffer);
	}

	pub fn event(&self, event: Event) {
		self.events.borrow_mut().push(browser::Event::Buffer(
			self.buffer.borrow().as_ref().unwrap().clone(), event));
	}

	pub fn resize(&self, width: u32, height: u32) {
		self.size.set((width, height));
	}

	pub fn create_window_proxy(&self) -> glutin::WindowProxy {
		self.glutin.create_window_proxy()
	}
}

impl WindowMethods for Window {
	fn create_compositor_channel(window: &Option<Rc<Window>>) -> (Box<CompositorProxy + Send>, Box<CompositorReceiver>) {
		Proxy::new(window)
	}

	fn supports_clipboard(&self) -> bool {
		true
	}

	fn hidpi_factor(&self) -> ScaleFactor<ScreenPx, DevicePixel, f32> {
		ScaleFactor::new(self.glutin.hidpi_factor())
	}

	fn framebuffer_size(&self) -> TypedSize2D<DevicePixel, u32> {
		let factor          = self.glutin.hidpi_factor() as u32;
		let (width, height) = self.size.get();

		Size2D::typed(width * factor, height * factor)
	}

	fn size(&self) -> TypedSize2D<ScreenPx, f32> {
		let (width, height) = self.size.get();

		Size2D::typed(width as f32, height as f32)
	}

	fn native_display(&self) -> NativeDisplay {
		unsafe {
			NativeDisplay::new(self.glutin.platform_display() as *mut Display)
		}
	}

	fn client_window(&self) -> (Size2D<u32>, Point2D<i32>) {
		let (width, height) = self.size.get();
		(Size2D::new(width, height), Point2D::zero())
	}

	fn set_inner_size(&self, size: Size2D<u32>) {
		self.event(Event::Resize(size));
	}

	fn set_position(&self, point: Point2D<i32>) {
		self.event(Event::Position(point));
	}

	fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
		true
	}

	fn present(&self) {
		self.event(Event::Present);
	}

	fn set_page_title(&self, title: Option<String>) {
		*self.title.borrow_mut() = title.clone();
		self.event(Event::Title(title));
	}

	fn set_page_url(&self, url: Url) {
		*self.url.borrow_mut() = Some(url.clone());
		self.event(Event::Url(url));
	}

	fn set_favicon(&self, url: Url) {
		*self.favicon.borrow_mut() = Some(url.clone());
		self.event(Event::Favicon(url));
	}

	fn set_cursor(&self, cursor: Cursor) {
		*self.cursor.borrow_mut() = Some(cursor);
		self.event(Event::Cursor(cursor));
	}

	fn status(&self, status: Option<String>) {
		*self.status.borrow_mut() = status.clone();
		self.event(Event::Status(status));
	}

	fn load_start(&self, back: bool, forward: bool) {
		*self.load.borrow_mut() = Some(Load::Start{ back: back, forward: forward });
		self.event(Event::Load(Load::Start { back: back, forward: forward }));
	}

	fn load_end(&self, back: bool, forward: bool) {
		*self.load.borrow_mut() = Some(Load::End { back: back, forward: forward });
		self.event(Event::Load(Load::End { back: back, forward: forward }));
	}

	// TODO: fix NetError because it's shit
	fn load_error(&self, _code: NetError, url: String) {
		*self.load.borrow_mut() = Some(Load::Error { url: url.clone() });
		self.event(Event::Load(Load::Error { url: url }));
	}

	fn head_parsed(&self) {
		self.event(Event::Head);
	}

	fn handle_key(&self, key: Key, modifier: KeyModifiers) {
		self.event(Event::Key { key: key, modifier: modifier });
	}
}

struct Proxy {
	sender: Sender<Msg>,
	proxy:  Option<glutin::WindowProxy>,
}

unsafe impl Send for Proxy { }

impl Proxy {
	pub fn new(window: &Option<Rc<Window>>) -> (Box<CompositorProxy + Send>, Box<CompositorReceiver>) {
		let (sender, receiver) = channel();

		let proxy = match window {
			&Some(ref window) => Some(window.create_window_proxy()),
			&None             => None,
		};

		(
			box Proxy {
				sender: sender,
				proxy:  proxy,
			} as Box<CompositorProxy + Send>,

			box receiver as Box<CompositorReceiver>
		)
	}
}

impl CompositorProxy for Proxy {
	fn send(&self, msg: Msg) {
		self.sender.send(msg).unwrap();

		if let Some(ref proxy) = self.proxy {
			proxy.wakeup_event_loop();
		}
	}

	fn clone_compositor_proxy(&self) -> Box<CompositorProxy + Send> {
		box Proxy {
			sender: self.sender.clone(),
			proxy:  self.proxy.clone(),
		} as Box<CompositorProxy + Send>
	}
}
