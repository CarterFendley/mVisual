use js_sys::{WebAssembly,Object};
use wasm_bindgen::JsCast;
use web_sys::*;

/*
This is the "tag trait" idiom, see here: https://stackoverflow.com/a/72523533/11325551
*/

pub trait SupportedTypes {}

impl SupportedTypes for u16 {}
impl SupportedTypes for f32 {}

pub fn fill_new_buffer<T: SupportedTypes + 'static>(gl: &WebGlRenderingContext, target: u32, vector: &Vec<T>, usage: u32) -> WebGlBuffer {
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
  let js_array: Object = if bytes == 2 {
    Object::from(js_sys::Uint16Array::new(&wasm_memory).subarray(
      location,
      location + vector.len() as u32
    ))
  } else {
    Object::from(js_sys::Float32Array::new(&wasm_memory).subarray(
      location,
      location + vector.len() as u32
    ))
  };
  
  // Load data into buffer
  gl.bind_buffer(target, Some(&buffer));
  gl.buffer_data_with_array_buffer_view(
    target,
    &js_array,
    usage
  );

  return buffer
}