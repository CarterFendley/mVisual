pub const GRID_SIZE: usize = 10;

pub const FIELD_OF_VIEW: f32 = 45. * std::f32::consts::PI / 180.; // In radians

// How far away and close do you stop rendering objects
pub const Z_FAR: f32 = 100.;
pub const Z_NEAR: f32 = 0.1;

/*
* How far the Z plane is away from you (based on the 45. deg FOV)
*/
pub const Z_PLANE: f32 = -2.414213; // -1 / tan(pi /8)