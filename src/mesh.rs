use crate::convert::{to_mat4};
use std::fs::OpenOptions;
use std::io::{Read, Seek};

pub trait WebGlTriangles {
	// Allows rendering with gl.TRIANGLES
	fn to_gl_triangles_vertices(&self) -> Vec<f32>;
	fn to_gl_triangles_indices(&self) -> Vec<u32>;
    fn to_model_view_mat4(&self) -> [f32; 16];
}


pub struct Cube {
	xmin : f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
	zmin: f32,
	zmax: f32,
    model_view: na::Matrix4<f32>
} 

impl Cube {
	pub fn new(xmin: f32, ymin: f32, zmin: f32, xmax: f32, ymax: f32, zmax: f32, model_view: na::Matrix4<f32>) -> Self {
		Cube {xmin, xmax, ymin, ymax, zmin, zmax, model_view}
	}
}

impl WebGlTriangles for Cube {
	fn to_gl_triangles_vertices(&self) -> Vec<f32> {
		return vec![
			self.xmin, self.ymin, self.zmin, // BLB 0
			self.xmin, self.ymin, self.zmax, // BLF 1
			self.xmin, self.ymax, self.zmin, // TLB 2
			self.xmin, self.ymax, self.zmax, // TLF 3 
			self.xmax, self.ymin, self.zmin, // BRB 4
			self.xmax, self.ymin, self.zmax, // BRF 5
			self.xmax, self.ymax, self.zmin, // TRB 6
			self.xmax, self.ymax, self.zmax, // TRF 7
		];
	}

	fn to_gl_triangles_indices(&self) -> Vec<u32> {
		return vec! [
			1, 3, 7,      1, 5, 7,    // Front face
			0, 2, 6,      0, 4, 6,    // Back face
			2, 3, 7,      2, 6, 7,    // Top face
			0, 1, 5,      0, 4, 5,    // Bottom face
			4, 5, 7,      4, 6, 7,    // Right face
			0, 1, 3,      0, 2, 3     // Left face
	  ];
	}

    fn to_model_view_mat4(&self) -> [f32; 16] {
        to_mat4(&self.model_view)
    }
}

pub struct Mesh {
    vertices: Vec<f32>,
    triangles: Vec<u32>,
    model_view: na::Matrix4<f32>
}


impl Mesh {
    pub fn from_stl<F: Read + Seek>(file: F, model_view: na::Matrix4<f32>) -> Self {
        let stl = stl_io::read_stl(&mut file).unwrap();


        let vertices = Vec::new();
        let triangles = Vec::new();

        return Mesh {
            vertices, triangles, model_view
        }
    }
}

impl WebGlTriangles for Mesh {
	fn to_gl_triangles_vertices(&self) -> Vec<f32> {
		return self.vertices.clone(); 
	}

	fn to_gl_triangles_indices(&self) -> Vec<u32> {
		return self.triangles.clone();
	}

    fn to_model_view_mat4(&self) -> [f32; 16] {
        to_mat4(&self.model_view)
    }
}
