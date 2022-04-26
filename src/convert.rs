pub fn to_mat4(m: &na::Matrix4<f32>) -> [f32; 16] {
    return flatten(m.clone().into())
}

// This is annoying
fn flatten(a: [[f32; 4]; 4]) -> [f32; 16] {
    unsafe { std::mem::transmute(a) }
}

