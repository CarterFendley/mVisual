use super::constants::*;
//use super::super::log;
use nalgebra::{Matrix4,Perspective3};

pub fn get_3d_projection_matrix(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_angle_x_axis: f32,
    rotation_angle_y_axis: f32,
) -> Matrix4<f32> {
    // Matrices to rotate the objects around x and y axises
    let rotate_x_axis: [f32; 16] = [
        1.,
        0.,
        0.,
        0.,
        0.,
        rotation_angle_x_axis.cos(),
        -rotation_angle_x_axis.sin(),
        0.,
        0.,
        rotation_angle_x_axis.sin(),
        rotation_angle_x_axis.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ];
    let rotate_y_axis: [f32; 16] = [
        rotation_angle_y_axis.cos(),
        0.,
        rotation_angle_y_axis.sin(),
        0.,
        0.,
        1.,
        0.,
        0.,
        -rotation_angle_y_axis.sin(),
        0.,
        rotation_angle_y_axis.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ];

    // Pack the transformations together
    let rotation_matrix = mult_matrix_4(rotate_x_axis, rotate_y_axis);

    let aspect_ratio: f32 = canvas_width / canvas_height;
    let scale_x = (right - left) / canvas_width;
    let scale_y = (top - bottom) / canvas_height;
    let scale_generic = scale_y;

    /*
     * Says the the "scale" stuff is only needed to make 2d work with 3d well.
     *
     * If we only had 3d wouldn't need to scale?
     *
     * See: https://youtu.be/K63uBfs1K7Y?t=1321
     * TODO: Check this
     */
    let translation_matrix: [f32; 16] = translation_matrix(
        -1. + scale_x + 2. * left / canvas_width,
        -1. + scale_y + 2. * bottom / canvas_height,
        Z_PLANE, // Confusing
    );

    let scale_matrix: [f32; 16] = scale_matrix(scale_generic, scale_generic, 0.);
    let rotation_scale = mult_matrix_4(rotation_matrix, scale_matrix);
    let combined_transform = mult_matrix_4(rotation_scale, translation_matrix);
    let mv = Matrix4::new(
        combined_transform[0],
        combined_transform[4],
        combined_transform[8],
        combined_transform[12],
        combined_transform[1],
        combined_transform[5],
        combined_transform[9],
        combined_transform[13],
        combined_transform[2],
        combined_transform[6],
        combined_transform[10],
        combined_transform[14],
        combined_transform[3],
        combined_transform[7],
        combined_transform[11],
        combined_transform[15],
    );

    /*
     * This part gives it the "3D" feel
     */
    let perspective: Perspective3<f32> =
        Perspective3::new(aspect_ratio, FIELD_OF_VIEW, Z_NEAR, Z_FAR);

    return perspective.as_matrix() * mv;
}

/**
 * Generates positions + indices for a given size (or resolution) of a grid.
 *
 * Size (n) is measured in squares on the grid. 1x1 being a single square.
 *
 * For a 1x1 grid you need 2x2 to specify all the vertices on that grid. For a 2x2 grid you need a 3x3.
 * See: https://youtu.be/K63uBfs1K7Y?t=490
 *
 * Vectors used so we don't have to know the size at compile time.
 */
pub fn get_position_grid_n_by_n(n: usize) -> (Vec<f32>, Vec<u16>) {
    let n_plus_one = n + 1;

    /*
     * Create a 2d grid containing X,Y,Z data in a vector.
     *
     * For 2x2 grid (3x3 vertices) this will be
     *
     * Row Z=0: (X,Y,Z), (X,Y,Z), (X,Y,Z)
     * Row Z=1: (X,Y,Z), (X,Y,Z), (X,Y,Z)
     * Row Z=2: (X,Y,Z), (X,Y,Z), (X,Y,Z)
     *
     * He notes we could leave out the 3 since we are defining the y "later"
     */
    let mut positions: Vec<f32> = vec![0.; 3 * n_plus_one * n_plus_one];
    /*
     * Indices must be able to specify 6 vertices per square to have two triangles cover each part of the grid.
     */
    let mut indices: Vec<u16> = vec![0; 6 * n * n];

    // Mimicking webGL's -1 to 1 (nothing is predefined though as we are transforming our points!)
    let graph_width: f32 = 2.;
    let square_size: f32 = graph_width / n as f32;

    for z in 0..n_plus_one {
        for x in 0..n_plus_one {
            /*
             * which 'row' = Z * n_plus_one
             */
            let start_pos_i = 3 * (z * n_plus_one + x);

            // Again -1 and square size is mimicking openGL grid, but could be something else
            positions[start_pos_i + 0] = -1. + (x as f32) * square_size;
            positions[start_pos_i + 1] = 0.;
            positions[start_pos_i + 2] = -1. + (z as f32) * square_size;

            /*
             * "Make sure that if either (x, y) are equal to n we don't do the calculations"
             *
             * Pretty sure this comes from the size difference between the two arrays. n+1 vs n. If we tried to do
             */
            if z < n && x < n {
                // Calculate the position in the indices vector we are at similar to how we did for the position.
                let start_index_i = 6 * (z * n + x);

                let vertex_index_top_left = (z * n_plus_one + x) as u16;
                let vertex_index_bottom_left = vertex_index_top_left + n_plus_one as u16;
                let vertex_index_top_right = vertex_index_top_left + 1;
                let vertex_index_bottom_right = vertex_index_bottom_left + 1;

                indices[start_index_i + 0] = vertex_index_top_left;
                indices[start_index_i + 1] = vertex_index_bottom_left;
                indices[start_index_i + 2] = vertex_index_bottom_right;
                indices[start_index_i + 3] = vertex_index_top_left;
                indices[start_index_i + 4] = vertex_index_bottom_right;
                indices[start_index_i + 5] = vertex_index_top_right;
            }
        }
    }

    return (positions, indices);
}

/*
 16 acting as 4x4 matrix here
*/
pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    // Make an identity matrix by putting ones on the diagonal
    return_var[0] = 1.;
    return_var[5] = 1.;
    return_var[10] = 1.;
    return_var[15] = 1.;

    return_var[12] = tx;
    return_var[13] = ty;
    return_var[14] = tz;

    return return_var;
}

pub fn scale_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = sx;
    return_var[5] = sy;
    return_var[10] = sz;
    return_var[15] = 1.;

    return return_var;
}

/*
    Could be replaced by some lib / built in to multiple two maticies
*/
pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return return_var;
}
