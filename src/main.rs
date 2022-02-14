use web_sys::console;
mod webgl;


fn console_log(s: String){
	console::log_1(&s.into());
}


pub trait WebGLTriangles {
	// Allows rendering with gl.TRIANGLES
	fn to_gl_triangles_vertices(&self) -> Vec<f32>;
}

pub struct Cube {
	xmin : f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
	zmin: f32,
	zmax: f32
} 

impl Cube {
	pub fn new(xmin: f32, ymin: f32, zmin: f32, xmax: f32, ymax: f32, zmax: f32) -> Self {
		Cube {xmin, xmax, ymin, ymax, zmin, zmax}
	}
}

impl WebGLTriangles for Cube {
	fn to_gl_triangles_vertices(&self) -> Vec<f32> {
		return vec![
			// Front
			self.xmin, self.ymin, self.zmax,
			self.xmax, self.ymin, self.zmax,
			self.xmax, self.ymax, self.zmax,
			self.xmin, self.ymax, self.zmax,
			// Back	
			self.xmin, self.ymin, self.zmin,
			self.xmax, self.ymin, self.zmin,
			self.xmax, self.ymax, self.zmin,
			self.xmin, self.ymax, self.zmin,
			// Top
			// Bottom
			// Left
			// Right
		];
	}
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

	let width = 600;
	let height = 400;

	let c = Cube::new(-0.8, -0.8, -0.8, 0.8, 0.8, 0.8);

	let vertices = c.to_gl_triangles_vertices();
	console_log(format!("Rendering {} vertices...", vertices.len()));
    let context = webgl::create_webgl_pane(width, height).expect("Couldn't create webgl");
	let program = webgl::get_basic_webgl_program(&context);
	webgl::set_uniform1f(&context, &program, "width", width as f32);
	webgl::set_uniform1f(&context, &program, "height", height as f32);
    webgl::clear(&context);
	webgl::draw_triangles(&context, &program, &vertices, "position");


}


