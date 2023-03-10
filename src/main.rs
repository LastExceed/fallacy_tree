#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use fallacy_tree::TreeBrowser;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
	// Log to stdout (if you run with `RUST_LOG=debug`).
	tracing_subscriber::fmt::init();

	eframe::run_native(
		"fallacy tree",
		eframe::NativeOptions::default(),
		Box::new(|_creation_context| Box::new(TreeBrowser::new())),
	);
}

#[cfg(target_arch = "wasm32")]
fn main() {
	// Make sure panics are logged using `console.error`.
	console_error_panic_hook::set_once();

	// Redirect tracing to console.log and friends:
	tracing_wasm::set_as_global_default();

	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		eframe::start_web(
			"the_canvas_id", // hardcode it
			web_options,
			Box::new(|_creation_context| Box::new(TreeBrowser::new())),
		)
			.await
			.expect("failed to start eframe");
	});
}