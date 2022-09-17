extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod programs;
mod shaders;
mod util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct MVisual {
    gl: WebGlRenderingContext,
    programs: Vec<Box<dyn programs::Program>>,
}

#[wasm_bindgen]
impl MVisual {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = util::webgl::initialize_webgl_context().unwrap();

        let mut programs: Vec<Box<dyn programs::common::Program>> = Vec::new();
        // programs.push(Box::new(programs::Graph3D::new(&gl)));
        programs.push(Box::new(programs::Sphere3D::new(&gl, false)));

        Self {
            gl: gl,
            programs: programs,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dyanmic_data(time, height, width);
        return Ok(());
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        for program in self.programs.iter() {
            program.render(
                &self.gl,
                &curr_state,
            );
        }
    }
}
