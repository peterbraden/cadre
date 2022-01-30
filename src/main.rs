use web_sys::console;
use web_sys::{WebGl2RenderingContext};
mod webgl;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    println!("Hello, world!");
    console::log_1(&"Hello using web-sys".into());

	let width = 600;
	let height = 400;
    let context = webgl::create_webgl_pane(width, height).expect("Couldn't create webgl");
	let vert_shader = webgl::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
            gl_Position = position;
        }
        "##,
    ).expect("couldn't create shader");

    let frag_shader = webgl::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;
        out vec4 outColor;
		uniform float width;
		uniform float height;
        
        void main() {
			vec2 u_resolution = vec2(width, height);
			vec2 st = gl_FragCoord.xy / u_resolution;
            outColor = vec4(st.xy, 1, 1.0);
        }
        "##,
    ).expect("couldn't create shader");

    let program = webgl::link_program(&context, &vert_shader, &frag_shader).expect("couldn't link program");
    context.use_program(Some(&program));

	webgl::set_uniform1f(&context, &program, "width", width as f32);
	webgl::set_uniform1f(&context, &program, "height", height as f32);

	let vertices: [f32; 18] = [
		-1.0, -0.8, 0.0,
		 -1.0, 1.0, 0.0,
		 1.0, 1.0, 0.0,

		-1.0, -1.0, 0.0,
		 1.0, -1.0, 0.0,
		 1.0, 1.0, 0.0
	];
    webgl::clear(&context);
	webgl::draw_triangles(&context, &program, &vertices, "position");


}


