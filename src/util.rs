use sutil::opts;
use net_traits::hosts;
use script::dom::bindings::codegen::RegisterBindings;
use env_logger;
use script;

use num_cpus;

pub fn init() {
	env_logger::init().unwrap();
	hosts::global_init();
	script::init();
	RegisterBindings::RegisterProxyHandlers();

	let mut opts = opts::default_opts();

	opts.user_agent = concat!("Mozilla/5.0 Servo/1.0 miserve/",
		env!("CARGO_PKG_VERSION")).to_owned();

	opts.url            = None;
	opts.resources_path = None;

	opts.headless  = false;
	opts.hard_fail = false;

	opts.enable_text_antialiasing   = true;
	opts.enable_canvas_antialiasing = true;

	{
		let mut threads = num_cpus::get() * 3 / 4;

		if threads < 1 {
			threads = 1;
		}

		opts.paint_threads  = threads;
		opts.layout_threads = threads;
	}

	opts::set_defaults(opts);
}
