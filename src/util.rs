use sutil::opts::{self, Opts};
use net_traits::hosts;
use script::dom::bindings::codegen::RegisterBindings;
use url::Url;
use euclid::size::Size2D;
use env_logger;
use script;

pub fn init() {
	env_logger::init().unwrap();
	hosts::global_init();
	script::init();
	RegisterBindings::RegisterProxyHandlers();

	opts::set_defaults(Opts {
		url: Some(Url::parse("about:blank").unwrap()),
		paint_threads: 1,
		gpu_painting: false,
		tile_size: 512,
		device_pixels_per_px: None,
		time_profiler_period: None,
		mem_profiler_period: None,
		enable_experimental: false,
		layout_threads: 1,
		nonincremental_layout: false,
		nossl: false,
		userscripts: None,
		user_stylesheets: Vec::new(),
		output_file: None,
		replace_surrogates: false,
		gc_profile: false,
		headless: true,
		hard_fail: true,
		bubble_inline_sizes_separately: false,
		show_debug_borders: false,
		show_debug_fragment_borders: false,
		show_debug_parallel_paint: false,
		show_debug_parallel_layout: false,
		paint_flashing: false,
		enable_text_antialiasing: false,
		enable_canvas_antialiasing: false,
		trace_layout: false,
		devtools_port: None,
		webdriver_port: None,
		initial_window_size: Size2D::typed(800, 600),
		user_agent: concat!("Mozilla/5.0 Servo/1.0 miserve/", env!("CARGO_PKG_VERSION")).to_owned(),
		multiprocess: false,
		dump_flow_tree: false,
		dump_display_list: false,
		dump_display_list_json: false,
		dump_display_list_optimized: false,
		relayout_event: false,
		validate_display_list_geometry: false,
		profile_tasks: false,
		resources_path: None,
		sniff_mime_types: false,
		disable_share_style_cache: false,
		parallel_display_list_building: false,
		exit_after_load: false,
	});
}
