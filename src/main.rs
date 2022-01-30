use web_sys::console;
mod webgl;


fn console_log(s: String){
	console::log_1(&s.into());
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

	let width = 600;
	let height = 400;

	let vertices: [f32; 18] = [
		-1.0, -0.8, 0.0,
		 -1.0, 1.0, 0.0,
		 1.0, 1.0, 0.0,

		-1.0, -1.0, 0.0,
		 1.0, -1.0, 0.0,
		 1.0, 1.0, 0.0
	];

	console_log(format!("Rendering {} vertices...", vertices.len()));
    let context = webgl::create_webgl_pane(width, height).expect("Couldn't create webgl");
	let program = webgl::get_basic_webgl_program(&context);
	webgl::set_uniform1f(&context, &program, "width", width as f32);
	webgl::set_uniform1f(&context, &program, "height", height as f32);
    webgl::clear(&context);
	webgl::draw_triangles(&context, &program, &vertices, "position");


}


