#![allow(unused_imports)]
#![allow(dead_code)]

mod webgl;
extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::webgl::{RenderContext};

/*
use web_sys::console;
fn console_log(s: String){
	console::log_1(&s.into());
}
*/


fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
	let width = 600;
	let height = 400;

	let eye    = na::Point3::new(0.0, 10.0, 10.0);
	let target = na::Point3::new(0.0, 0.0, 0.0);
	let view   = na::Isometry3::look_at_rh(&eye, &target, &Vector3::y());
	let projection: na::Perspective3<f32>  = na::Perspective3::new(width as f32/height as f32, 0.4, 0.1, 10000.);

	let mut rc = RenderContext::new(width, height, projection.as_matrix() * view.to_homogeneous());

	rc.add_object(
		Box::new(
			webgl::Cube::new(-0.3, -0.3, -0.3, 0.6, 0.8, 0.8, na::Matrix4::identity())
		)
	);
	rc.start();
}

