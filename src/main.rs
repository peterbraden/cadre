use web_sys::console;
mod webgl;
extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::webgl::{RenderContext};

fn console_log(s: String){
	console::log_1(&s.into());
}


fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

	let mut rc = RenderContext::new(600, 400, na::Perspective3::new(600.0/400.0, 10.0, 0.1, 3.0).into_inner());

	let axisangle = Vector3::y() * std::f32::consts::FRAC_PI_2;
	let model_view = Rotation3::new(axisangle).to_homogeneous();

	rc.add_object(
		Box::new(
			webgl::Cube::new(-0.3, -0.3, -0.3, 0.6, 0.8, 0.8, model_view)
		)
	);
	rc.start();
}

