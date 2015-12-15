use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use servo::net::image_cache_task::new_image_cache_task;
use servo::net::resource_task::new_resource_task;
use servo::net::storage_task::StorageTaskFactory;
use servo::gfx::font_cache_task::FontCacheTask;
use servo::profile;

pub use servo::net_traits::ResourceTask as Resource;
pub use servo::net_traits::storage_task::StorageTask as Storage;
pub use servo::net_traits::image_cache_task::ImageCacheTask as ImageCache;
pub use servo::gfx::font_cache_task::FontCacheTask as FontCache;
pub use servo::profile_traits::time::ProfilerChan as TimeProfiler;
pub use servo::profile_traits::mem::ProfilerChan as MemProfiler;

use servo::util::opts;

use servo::gl;
use glutin;

use browser::{buffer, Buffer, Tab, Event};

/// This struct is the main entry point of the browser.
///
/// When rendering it will render the tab bar, the input bar and the current tab
/// to the given textures, and composite them back onto the screen.
pub struct Browser {
	window: Rc<glutin::Window>,
	events: Rc<RefCell<Vec<Event>>>,

	profiler: Profiler,
	cache:    Cache,
	resource: Resource,
	storage:  Storage,

	buffers:  Vec<Rc<RefCell<Buffer>>>,
	tabs:     Vec<Rc<RefCell<Tab>>>,
	current:  Option<Rc<RefCell<Tab>>>,
}

pub struct Profiler {
	time: Option<f64>,
	mem:  Option<f64>,
}

impl Profiler {
	pub fn time(&self) -> TimeProfiler {
		profile::time::Profiler::create(self.time)
	}

	pub fn mem(&self) -> MemProfiler {
		profile::mem::Profiler::create(self.mem)
	}
}

pub struct Cache {
	image: ImageCache,
	font:  FontCache,
}

impl Cache {
	pub fn image(&self) -> ImageCache {
		self.image.clone()
	}

	pub fn font(&self) -> FontCache {
		self.font.clone()
	}
}

impl Browser {
	pub fn new(window: Rc<glutin::Window>) -> Browser {
		let opts     = opts::get();
		let resource = new_resource_task(opts.user_agent.clone(), None);

		gl::load_with(|s| window.get_proc_address(s) as *const _);
		
		Browser {
			window: window,
			events: Rc::new(RefCell::new(Vec::new())),

			profiler: Profiler {
				time: opts.time_profiler_period,
				mem:  opts.mem_profiler_period,
			},

			cache: Cache {
				image: new_image_cache_task(resource.clone()),
				font:  FontCacheTask::new(resource.clone()),
			},

			resource: resource,
			storage:  StorageTaskFactory::new(),

			buffers: Vec::new(),
			tabs:    Vec::new(),
			current: None,
		}
	}

	pub fn show(&mut self) {
		if self.tabs.is_empty() {
			let tab = Rc::new(RefCell::new(Tab::new(self)));
			self.tabs.push(tab.clone());
			self.current = Some(tab);
		}

		self.window.show();
	}

	pub fn window(&self) -> Rc<glutin::Window> {
		self.window.clone()
	}

	pub fn events(&self) -> Rc<RefCell<Vec<Event>>> {
		self.events.clone()
	}

	pub fn width(&self) -> u32 {
		self.window.get_inner_size_pixels().unwrap().0
	}

	pub fn height(&self) -> u32 {
		self.window.get_inner_size_pixels().unwrap().1
	}

	pub fn resource(&self) -> Resource {
		self.resource.clone()
	}

	pub fn storage(&self) -> Storage {
		self.storage.clone()
	}

	pub fn profiler(&self) -> &Profiler {
		&self.profiler
	}

	pub fn cache(&self) -> &Cache {
		&self.cache
	}

	pub fn handle(&mut self, event: Event) {
		let mut events = mem::replace(&mut *self.events.borrow_mut(), Vec::new());
		events.push(event);

		for event in events {
			match event {
				Event::Idle => {
					for buffer in &self.buffers {
						buffer.borrow_mut().idle();
					}
				}

				Event::Window(glutin::Event::Refresh) => {
					self.render();
				}

				// should render only if the buffer is currently in view
				Event::Buffer(_, buffer::Event::Load(buffer::Load::End { .. })) => {
					self.render();
				}

				_ => ()
			}
		}
	}

	pub fn render(&mut self) {
		if let Some(tab) = self.current.as_ref() {
			tab.borrow_mut().render();
		}

		self.window.swap_buffers().unwrap();
	}

	// XXX: temporary to test
	pub fn open(&mut self, url: ::servo::url::Url, (width, height): (u32, u32), (x, y): (u32, u32)) {
		let buffer = Buffer::new(self);
		buffer.borrow_mut().go(url);
		self.buffers.push(buffer.clone());

		if let Some(tab) = self.current.as_ref() {
			let window = tab.borrow_mut().open();

			window.borrow_mut().assign(buffer);
			window.borrow_mut().resize(width, height);
			window.borrow_mut().position(x, y);
		}
	}
}
