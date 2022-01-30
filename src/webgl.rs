/// Utilities for rendering to a webgl canvas element
use js_sys;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub fn create_webgl_pane(width: u32, height: u32) -> Result<WebGl2RenderingContext, String>{
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().expect("Document needs body");
    let canvas = document.create_element("canvas").expect("Can't create canvas");
    canvas.set_id("webgl");
    body.append_with_node_1(&canvas).expect("could not append canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Couldn't convert to HtmlCanvas");
    let _ = canvas.style().set_property("width", &(width.to_string() + "px"));
    let _ = canvas.style().set_property("height", &(height.to_string() + "px"));
    let _ = canvas.style().set_property("background-color", "#222");
    let context = canvas.get_context("webgl2").unwrap().expect("Couldn't get webgl2 context").dyn_into::<WebGl2RenderingContext>().expect("Couldn't cast");
    return Ok(context);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn clear(context: &WebGl2RenderingContext) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

/// Quick and simple way of drawing a mesh
/// - Unindexed vertices (uses WebGL draw_arrays)
/// - Assume [x, y, z, x, y, z] layout.
pub fn draw_triangles(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    vertices: &[f32],
    name: &str
) {
    let vao = context
        .create_vertex_array()
        .expect("Could not create vertex array object");
    context.bind_vertex_array(Some(&vao));

    let buffer = context.create_buffer().expect("Failed to create buffer");
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let position_attribute_location = context.get_attrib_location(&program, name);
    context.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    let vert_count = (vertices.len() / 3) as i32;
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn set_uniform1f(context: &WebGl2RenderingContext, program: &WebGlProgram, name: &str, value: f32) {
    let location = context.get_uniform_location(program, name);
    if location.is_some() {
        context.uniform1f(Some(&location.unwrap()), value);
    }
}
