#![feature(box_syntax, result_expect)]
#![allow(dead_code, unused_variables)]

extern crate compositing;
extern crate net;
extern crate net_traits;
extern crate script;
extern crate script_traits;
extern crate profile;
extern crate profile_traits;
extern crate layers;
extern crate msg;
extern crate layout;
extern crate gfx;
extern crate util as sutil;

extern crate glutin;
extern crate gleam;
extern crate x11;

use glutin::Event;

extern crate time;
extern crate url;
extern crate euclid;

use url::Url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate bitflags;

use std::rc::Rc;
use std::env;

pub mod util;
use util::init;

pub mod window;
use window::Window;
use compositing::windowing::WindowMethods;

pub mod browser;
use browser::Browser;

fn main() {
	util::init();

	let     window  = Rc::new(Window::new());
	let mut browser = Browser::new(window.clone());

	browser.go(Url::parse(&env::args().nth(1).expect("no url given")).unwrap());

	'main: loop {
		for event in window.wait_events() {
			if let &Event::Closed = &event {
				break 'main;
			}

			if !browser.handle(event) {
				break 'main;
			}

			window.present();
		}
	}

	browser.shutdown();
}
