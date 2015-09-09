#![feature(box_syntax, result_expect)]
#![allow(dead_code, unused_variables)]

extern crate servo;
use servo::gl;

extern crate x11;
extern crate glutin;
use glutin::{Event, GlRequest};

extern crate time;

use servo::url::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate bitflags;

extern crate num_cpus;

use std::env;
use std::rc::Rc;
use std::thread;

pub mod util;
use util::init;

pub mod browser;
pub use browser::Browser;

fn main() {
	util::init();

	let window = Rc::new(glutin::WindowBuilder::new()
		.with_title("miserve".to_owned())
		.with_dimensions(800, 600)
		.with_visibility(false)
		.with_gl(GlRequest::Latest)
		.build()
		.unwrap());

	unsafe {
		window.make_current().expect("failed to make the context current");

		gl::load_with(|s| window.get_proc_address(s));

		gl::clear_color(0.6, 0.6, 0.6, 1.0);
		gl::clear(gl::COLOR_BUFFER_BIT);
		gl::finish();

		window.swap_buffers().unwrap();
	}

	let mut browser = Browser::new(window.clone());
	browser.show();

	browser.open(Url::parse(&env::args().nth(1).expect("no url given")).unwrap(), (400, 600), (0, 0));
	browser.open(Url::parse(&env::args().nth(2).expect("no url given")).unwrap(), (400, 600), (400, 0));

	let mut events = window.poll_events();

	'main: loop {
		if let Some(event) = events.next() {
			if let &Event::Closed = &event {
				break 'main;
			}

			browser.handle(browser::Event::Window(event));
		}
		else {
			browser.handle(browser::Event::Idle);
			thread::sleep_ms(10);
		}
	}
}
