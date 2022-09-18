
use js_sys::{WebAssembly,Object};

use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn fill_new_buffer<T: 'static>(gl: &WebGlRenderingContext, vector: &Vec<T>) -> WebGlBuffer {
  let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();

  // Get location of data as an index
  let mut bytes = 4;
  if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u16>() {
    bytes = 2;
  }
  let location = vector.as_ptr() as u32 / bytes;

  // Move data to JS readable format
  let wasm_memory = wasm_bindgen::memory()
    .dyn_into::<WebAssembly::Memory>()
    .unwrap()
    .buffer();
  let mut js_array: js_sys::JsObject = if bytes == 2 {
    js_sys::Uint16Array::new(&wasm_memory);
  } else {
    js_sys::Float32Array::new(&wasm_memory);
  };
  js_array = js_array.subarray(location, location + vector.len() as u32);
  
  // Load data into buffer
  gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
  gl.buffer_data_with_array_buffer_view(
    GL::ARRAY_BUFFER,
    &js_array,
    GL::STATIC_DRAW
  );

  return buffer
}