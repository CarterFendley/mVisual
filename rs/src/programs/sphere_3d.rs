use super::super::util::Sphere;
use super::super::util::constants::*;
use super::super::util::webgl;
use super::super::log;
use super::common::Program;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use nalgebra::{Perspective3, Matrix4, Vector3};

pub struct Sphere3D {
  program: WebGlProgram,
  mv_matrix: Matrix4<f32>,
  // Uniform locations
  u_amb_light_color: WebGlUniformLocation, // TODO: Do these need to be set in render? (the lookup references the program...)
  u_diff_light_color: WebGlUniformLocation,
  u_diff_light_pos: WebGlUniformLocation,
  u_material_color: WebGlUniformLocation,
  u_mv_transform: WebGlUniformLocation,
  u_mvp_transform: WebGlUniformLocation,
  // Attribute locations
  a_vertex_position: u32,
  a_vertex_normal: u32,
  buf_vertex_position: WebGlBuffer,
  buf_vertex_normal: WebGlBuffer,
  buf_wireframe_indices: WebGlBuffer,
  len_wireframe_indices: i32,
}

impl Sphere3D {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
    let program = webgl::link_program(
      &gl,
      &super::super::shaders::vertex::sphere_3d::SHADER,
      &super::super::shaders::fragment::vary_color_from_vertex::SHADER,
    ).unwrap();

    let sphere = Sphere::new(10., 0);
    let model_matrix = Matrix4::new_scaling(10.);

    // Build view and invert
    let mut view_matrix = Matrix4::new_translation(
      &Vector3::new(0., 0., Z_PLANE)
    );
    match view_matrix.try_inverse() {
      Some(inv) => {
        view_matrix = inv
      }
      None => {}
    }

    log(&format!("Verticies: {}  Normals: {} Indices: {}", sphere.vertices.len(), sphere.normals.len(), sphere.wireframe_indices.len()));

    // Buffer any data that will remain unchaged
    let vertex_wasm_memory = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let vertex_location = sphere.vertices.as_ptr() as u32 / 4;
    let vertex_js_array = js_sys::Float32Array::new(&vertex_wasm_memory).subarray(
      vertex_location,
      vertex_location + sphere.vertices.len() as u32
    );
    let vertex_gpu_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_gpu_buffer));
    gl.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &vertex_js_array,
      GL::STATIC_DRAW,
    );

    let normals_wasm_memory = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let normals_location = sphere.normals.as_ptr() as u32 / 4;
    let normals_js_array = js_sys::Float32Array::new(&normals_wasm_memory).subarray(
      normals_location,
      normals_location + sphere.normals.len() as u32
    );
    let normals_gpu_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normals_gpu_buffer));
    gl.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &normals_js_array,
      GL::STATIC_DRAW
    );

    let widx_wasm_memory = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let widx_location = sphere.wireframe_indices.as_ptr() as u32 / 2;
    let widx_js_array = js_sys::Uint16Array::new(&widx_wasm_memory).subarray(
      widx_location,
      widx_location + sphere.wireframe_indices.len() as u32
    );
    let widx_gpu_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&widx_gpu_buffer));
    gl.buffer_data_with_array_buffer_view(
      GL::ELEMENT_ARRAY_BUFFER,
      &widx_js_array,
      GL::STATIC_DRAW
    );

    return Self {
      // Do everything that needs &program first
      u_amb_light_color: gl.get_uniform_location(&program, "uAmbientLightColor").unwrap(),
      u_diff_light_color: gl.get_uniform_location(&program, "uDiffuseLightColor").unwrap(),
      u_diff_light_pos: gl.get_uniform_location(&program, "uDiffuseLightPosition").unwrap(),
      u_material_color: gl.get_uniform_location(&program, "uMaterialColor").unwrap(),
      u_mv_transform: gl.get_uniform_location(&program, "uModelView").unwrap(),
      u_mvp_transform: gl.get_uniform_location(&program, "uModelViewProjection").unwrap(),
      a_vertex_position: gl.get_attrib_location(&program, "aVertexPosition") as u32,
      a_vertex_normal: gl.get_attrib_location(&program, "aVertexNormal") as u32,
      // Transfer program owner ship and finish
      program: program,
      mv_matrix: view_matrix * model_matrix,
      buf_vertex_position: vertex_gpu_buffer,
      buf_vertex_normal: normals_gpu_buffer,
      buf_wireframe_indices: widx_gpu_buffer,
      len_wireframe_indices: widx_js_array.length() as i32
    }
  }
}

impl Program for Sphere3D {
  fn render(
    &self,
    gl: &WebGlRenderingContext,
    _bottom: f32,
    _top: f32,
    _left: f32,
    _right: f32,
    canvas_height: f32,
    canvas_width: f32,
    _rotation_angle_x_axis: f32,
    _rotation_angle_y_axis: f32,
  ) {
    let aspect_ratio = canvas_width / canvas_height;
    let projection_matrix = Perspective3::new(
      aspect_ratio,
      FIELD_OF_VIEW,
      Z_NEAR,
      Z_FAR
    );
    gl.use_program(Some(&self.program));

    // Load uniforms
    gl.uniform3f(Some(&self.u_amb_light_color), 0.2, 0.2, 0.2); // Dim white
    gl.uniform3f(Some(&self.u_diff_light_color), 1., 1., 1.); // White
    gl.uniform3f(Some(&self.u_diff_light_pos), -0.85, 0.8, 0.75); // Above left shoulder
    gl.uniform3f(Some(&self.u_material_color), 0.5, 0.5, 0.8); // Blue-ish

    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.u_mv_transform),
      false,
      self.mv_matrix.as_slice()
    );
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.u_mvp_transform),
      false,
      (projection_matrix.as_matrix() * self.mv_matrix).as_slice()
    );

    // Attach buffers to attributes
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buf_vertex_position));
    gl.enable_vertex_attrib_array(self.a_vertex_position);
    gl.vertex_attrib_pointer_with_i32(self.a_vertex_position, 3, GL::FLOAT, false, 0, 0);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buf_vertex_normal));
    gl.enable_vertex_attrib_array(self.a_vertex_normal);
    gl.vertex_attrib_pointer_with_i32(self.a_vertex_normal, 3, GL::FLOAT, false, 0, 0);

    // Draw wireframe
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buf_wireframe_indices));
    gl.draw_elements_with_i32(GL::LINES, self.len_wireframe_indices, GL::UNSIGNED_SHORT, 0);
  }
}