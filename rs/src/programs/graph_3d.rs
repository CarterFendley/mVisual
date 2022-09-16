use super::super::util::math;
use super::super::util::webgl;
use super::super::util::constants::GRID_SIZE;
//use super::super::log;
use super::super::app_state::AppState;
use super::common::Program;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[allow(dead_code)]
pub struct Graph3D {
    pub program: WebGlProgram,
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub indices_buffer: WebGlBuffer,
    pub index_count: i32,
    pub position_buffer: WebGlBuffer,
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
}

#[allow(dead_code)]
impl Graph3D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = webgl::link_program(
            &gl,
            &super::super::shaders::vertex::graph_3d::SHADER,
            &super::super::shaders::fragment::vary_color_from_vertex::SHADER,
        )
        .unwrap();

        let positions_and_indices = math::get_position_grid_n_by_n(GRID_SIZE);

        // Bind data so webGL can use it
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = positions_and_indices.0.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + positions_and_indices.0.len() as u32,
        );
        let buffer_position = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_position));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = positions_and_indices.1.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + positions_and_indices.1.len() as u32,
        );
        let buffer_indices = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );

        Self {
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
            program: program,
            vertices: positions_and_indices.0,
            indices: positions_and_indices.1,
            indices_buffer: buffer_indices,
            index_count: indices_array.length() as i32,
            position_buffer: buffer_position,
        }
    }
}

impl Program for Graph3D {
    fn render(
        &self,
        gl: &WebGlRenderingContext,
        app_state: &AppState,
    ) {
        gl.use_program(Some(&self.program));

        // The name projection is odd to me as we have multiple matrices being returned not just the projection one.
        let projection_matrix = math::get_3d_projection_matrix(
            app_state.control_bottom,
            app_state.control_top,
            app_state.control_left,
            app_state.control_right,
            app_state.canvas_height,
            app_state.canvas_width,
            app_state.rotation_x_axis,
            app_state.rotation_y_axis,
        );

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection_matrix.as_slice());
        gl.uniform1f(Some(&self.u_opacity), 1.);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        /*
        // For debugging to add a visual indicator of a specific point
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));
        gl.draw_elements_with_i32(GL::POINTS, 1, GL::UNSIGNED_SHORT, 0);
        */

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0)
    }
}
