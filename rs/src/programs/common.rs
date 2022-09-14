use web_sys::WebGlRenderingContext;

pub trait Program {
    fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_angle_x: f32,
        rotation_angle_y: f32,
    );
}
