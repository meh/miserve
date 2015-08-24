use std::rc::Rc;

use compositing::CompositorEventListener;
use compositing::windowing::{WindowEvent, WindowMethods};
use glutin::Event;

use compositing::Constellation;
use compositing::CompositorTask;
use msg::constellation_msg::{Msg, ConstellationChan};
use layout::layout_task::LayoutTask;
use script::script_task::ScriptTask;

use euclid::size::Size2D;
use url::Url;

use profile;

use net::image_cache_task::new_image_cache_task;
use net::resource_task::new_resource_task;
use net::storage_task::StorageTaskFactory;
use gfx::font_cache_task::FontCacheTask;

use sutil::opts;
use window::Window;

pub struct Browser {
	compositor:    Box<CompositorEventListener + 'static>,
	constellation: ConstellationChan,
}

impl Browser {
	pub fn new(window: Rc<Window>) -> Browser {
		let opts          = opts::get();
		let has_clipboard = window.supports_clipboard();


		let (proxy, receiver) = WindowMethods::create_compositor_channel(&Some(window.clone()));
		let time_profiler     = profile::time::Profiler::create(opts.time_profiler_period);
		let mem_profiler      = profile::mem::Profiler::create(opts.mem_profiler_period);

		let constellation = {
			let resource    = new_resource_task(opts.user_agent.clone(), None);
			let image_cache = new_image_cache_task(resource.clone());
			let font_cache  = FontCacheTask::new(resource.clone());
			let storage     = StorageTaskFactory::new();

			let constellation = Constellation::<LayoutTask, ScriptTask>::start(proxy.clone_compositor_proxy(),
				resource, image_cache, font_cache,
				time_profiler.clone(), mem_profiler.clone(), None,
				storage, has_clipboard);

			if let Some(ref url) = opts.url {
				constellation.0.send(Msg::InitLoadUrl(url.clone())).unwrap();
			}

			constellation
		};

		let mut compositor = CompositorTask::create(Some(window), proxy, receiver,
			constellation.clone(), time_profiler, mem_profiler);

		compositor.handle_events(vec![WindowEvent::InitializeCompositing]);

		Browser {
			compositor:    compositor,
			constellation: constellation,
		}
	}

	pub fn handle(&mut self, event: Event) -> bool {
		let mut events = Vec::new();

		match event {
			Event::Refresh =>
				events.push(WindowEvent::Refresh),

			Event::Resized(width, height) =>
				events.push(WindowEvent::Resize(Size2D::typed(width, height))),

			_ => (),
		}

		self.compositor.handle_events(events)
	}

	pub fn go(&self, url: Url) {
		self.constellation.0.send(Msg::InitLoadUrl(url)).unwrap();
	}

	pub fn shutdown(mut self) {
		self.compositor.shutdown();
	}
}
