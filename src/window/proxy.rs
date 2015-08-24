use std::sync::mpsc::{channel, Sender};
use std::rc::Rc;

use compositing::compositor_task::{CompositorProxy, CompositorReceiver, Msg};
use glutin::WindowProxy;

pub struct Proxy {
	sender: Sender<Msg>,
	proxy:  Option<WindowProxy>,
}

unsafe impl Send for Proxy { }

impl Proxy {
	pub fn new(window: &Option<Rc<super::Window>>) -> (Box<CompositorProxy + Send>, Box<CompositorReceiver>) {
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
