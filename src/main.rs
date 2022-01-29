use web_sys::console;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    println!("Hello, world!");
    console::log_1(&"Hello using web-sys".into());

    create_webgl_pane();

}

fn create_webgl_pane() -> WebGl2RenderingContext{
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().expect("Document needs body");
    let canvas = document.create_element("canvas").expect("Can't create canvas");
    canvas.set_id("webgl");
    body.append_with_node_1(&canvas);
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("Couldn't convert to HtmlCanvas");
    canvas.style().set_property("width", "100vw");
    canvas.style().set_property("height", "100vh");
    canvas.style().set_property("background-color", "#222");

    let context = canvas.get_context("webgl2").unwrap().expect("Couldn't get webgl2 context").dyn_into::<WebGl2RenderingContext>().expect("Couldn't cast");
    return context;
}
