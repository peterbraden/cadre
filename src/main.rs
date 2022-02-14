use web_sys::console;
mod webgl;
extern crate nalgebra as na;
use na::{Vector3, Rotation3};

fn console_log(s: String){
	console::log_1(&s.into());
}

pub trait WebGLTriangles {
	// Allows rendering with gl.TRIANGLES
	fn to_gl_triangles_vertices(&self) -> Vec<f32>;
	fn to_gl_triangles_indices(&self) -> Vec<u32>;
}

// This is annoying
fn flatten(a: [[f32; 4]; 4]) -> [f32; 16] {
    unsafe { std::mem::transmute(a) }
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
			self.xmin, self.ymax, self.zmin,
			self.xmin, self.ymax, self.zmax,
			self.xmax, self.ymax, self.zmax,
			self.xmax, self.ymax, self.zmin,
			// Bottom
			// Left
			// Right

		];
	}

	fn to_gl_triangles_indices(&self) -> Vec<u32> {
		return vec! [
			0, 1, 2,      0, 2, 3,    // Front face
			4, 5, 6,      4, 6, 7,    // Back face
/*			8, 9, 10,     8, 10, 11,  // Top face
			12, 13, 14,   12, 14, 15, // Bottom face
			16, 17, 18,   16, 18, 19, // Right face
			20, 21, 22,   20, 22, 23  // Left face
*/
	  ];
	}
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

	let width = 600;
	let height = 400;

	let c = Cube::new(-0.3, -0.3, -0.3, 0.6, 0.8, 0.8);

	let vertices = c.to_gl_triangles_vertices();
	let indices = c.to_gl_triangles_indices();

    let context = webgl::create_webgl_pane(width, height).expect("Couldn't create webgl");
	let program = webgl::get_basic_webgl_program(&context);

	webgl::set_uniform1f(&context, &program, "width", width as f32);
	webgl::set_uniform1f(&context, &program, "height", height as f32);
	
	let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_2;
	let modelView: [f32; 16] = flatten(Rotation3::new(axisangle).to_homogeneous().into());
	webgl::set_uniform_mat4f(&context, &program, "modelView", &modelView);

	webgl::set_uniform_mat4f(&context, &program, "projection", &modelView);

    webgl::clear(&context);
	webgl::draw_triangles(&context, &program, &vertices, &indices, "position");
}


fn tick() {


}
