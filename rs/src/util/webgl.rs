use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    attach_mouse_down_handler(&canvas)?;
    attach_mouse_up_handler(&canvas)?;
    attach_mouse_move_handler(&canvas)?;

    /*
    The following will enable culling of faces that are pointed away the camera (must be defined counter clockwise)
    */
    gl.enable(GL::CULL_FACE);

    /*
    This will use the z value to deteremine which shapes should be shown or not depending if there is an object in front of them or not.

    https://learnopengl.com/Advanced-OpenGL/Depth-testing
    https://stackoverflow.com/a/43567650/11325551
    */
    gl.enable(GL::DEPTH_TEST);

    // What color should it set when it needs to erase something
    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA

    // Clear everything
    gl.clear_depth(1.);

    Ok(gl)
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        crate::app_state::update_mouse_down(
            event.client_x() as f32,
            event.client_y() as f32,
            true,
        )
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    /*
     * Tell rust to forget about the memory management of this function.
     *
     * We are sending it to JS, and will pass out of scope. If we don't tell
     * it to forget then it will be cleaned up / freed.
     *
     * Mini / one time memory leak on purpose;
     */
    handler.forget();

    return Ok(());
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        crate::app_state::update_mouse_down(
            event.client_x() as f32,
            event.client_y() as f32,
            false,
        )
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    /*
     * Tell rust to forget about the memory management of this function.
     *
     * We are sending it to JS, and will pass out of scope. If we don't tell
     * it to forget then it will be cleaned up / freed.
     *
     * Mini / one time memory leak on purpose;
     */
    handler.forget();

    return Ok(());
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        crate::app_state::update_mouse_position(
            event.client_x() as f32,
            event.client_y() as f32,
        );
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
