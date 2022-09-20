use crate::app_state::AppState;
use web_sys::WebGlRenderingContext;


pub trait Program {
    fn render(
        &self,
        gl: &WebGlRenderingContext,
        app_state: &AppState,
    );
}
