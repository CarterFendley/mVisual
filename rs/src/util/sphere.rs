use std::f32::consts::PI;

struct Sphere {
    radius: f32,
    v_sectors: u32,
    h_sectors: u32,
    vertices: Vec<f32>,
    normals: Vec<f32>,
    face_indices: Vec<u32>,
    wireframe_indices: Vec<u32>,
}

impl Sphere {
    pub fn new(radius: f32, resolution: u32) -> Self {
        /*
        The lower bound on the sectors bellow is based on the fact that the vertical sectors divide 180 degrees equally into each sector and horizontal ones do so with 360 degrees (see more in the recalculate method).

        In 180 degrees of space you need at least 2 sectors to get anything but a flat surface. In 360 degrees of space you need at least 3 to get more than just a two sided feature.
        */
        Self {
            radius: radius,
            v_sectors: 2 + resolution,
            h_sectors: 3 + resolution,
            vertices: Vec::new(),
            normals: Vec::new(),
            face_indices: Vec::new(),
            wireframe_indices: Vec::new()
        }
    }

    // https://stackoverflow.com/a/969880/11325551
    fn recalculate(&mut self) {
        // Scale for normalizing the vertex vectors
        let scale: f32 = 1. / self.radius;

        // Calculate the spacing between each sector
        let v_spacing = PI / (self.v_sectors as f32);
        let h_spacing = 2. * PI / (self.h_sectors as f32);

        /*
        Iterate through the sectors to calculate the vertex coordinates. The +1 is to make sure that h and v take on both h_sectors and v_sectors respectively. This will make sure we get the final PI and 2PI angles to finish off the bottom (vertically) / end (horizontally) of the sphere.

        ^ If it helps this is also because 0*x = 0;
        */
        for v in 0..(self.v_sectors + 1) {
            /*
            Vertical angle is on the range [PI/2, -PI/2] or [90deg, -90deg] hince the PI / 2 - PI below.

            Horizontal angles travel the full 360 deg. If vertical also went 360 deg not 180 deg we would calculate the same sector twice. Once on the front and once on the back of the circle. Could technically do this but then would want to bound horizontal angles to 180 deg because we would have computed all sectors by then.
            */
            let v_angle: f32 = PI / 2. - v as f32 * v_spacing;

            let z = self.radius * v_angle.sin();
            let z_norm = z * scale;
            // The hypotenuse of the x,y triangle
            let hypot = self.radius * v_angle.cos();

            for h in 0..(self.h_sectors + 1) {
                let h_angle = h as f32 * h_spacing;

                let x = hypot * h_angle.sin();
                let y = hypot * h_angle.sin();
                self.vertices.push(x);
                self.vertices.push(y);
                self.vertices.push(z);

                let x_norm = x * scale;
                let y_norm = y * scale;
                self.normals.push(x_norm);
                self.normals.push(y_norm);
                self.normals.push(z_norm);
            }
        }

        /*
        Now we split the sectors (each containing 4 vertices) into triangles. We stored the vertices in a one dimensional array like the following.

        [X1, Y1, Z1, X2, Y2, Z1]

        Row 0: * --- * .... * --- * (Top of circle)
               |     |      |     |
               |     |      |     |
        Row 1: * --- * .... * --- * (Next row of circle)
               :     :      :     :
               :     :      :     :
        Row v: * --- * .... * --- *

        In the linear array the bounds will be...
        For row 0: [0, `h_sectors`]
        For row 1: [`h_sectors+1`, `2*h_sectors`]

        This following loops are on the bounds [0, `v_sectors`) and [0, `h_sectors`) because we access the row below and the next horizontal location so all verticies in the range [0, `v_sectors`] and [0, `h_sectors`] will be used. If we looped from [..., ...] we would recieve an index out of bounds error.
        */
        for v in 0..self.v_sectors {
            let row_start = v * (self.h_sectors + 1);
            let row_below = row_start + (self.h_sectors + 1);
            // See comment below
            let last_row = v == (self.v_sectors - 1);

            for h in 0..self.h_sectors {
                // Get the corners of the sector
                let tl_corner = row_start + h;
                let tr_corner = tl_corner + 1;
                let bl_corner = row_below + h;
                let br_corner = bl_corner + 1;
                
                /*
                On row 0, the points are all the same (there is only one point at the top of the cirlce). But to keep our array balanced, and logic clean, we still have `h_sectors` entries for row 0. We can think of each sector as a triangle in this row.
                
                Row 0:    *     *
                         / \   / \
                        /   \ /   \
                Row 1: * --- * --- *

                If we treat this row like normal and try to draw 2 triangles, these triangles will overlap.

                The last row is also rendered similarly.
                */

                // Push the face_indices on in a counter clockwise orientation
                if v != 0 {
                    self.face_indices.push(tl_corner);
                    self.face_indices.push(bl_corner);
                    self.face_indices.push(tr_corner);
                }
                if !last_row {
                    self.face_indices.push(bl_corner);
                    self.face_indices.push(br_corner);
                    self.face_indices.push(tr_corner);
                }

                // For wireframe rendering
                self.wireframe_indices.push(tl_corner);
                self.wireframe_indices.push(bl_corner);
                // Top stack doesn't need a horizontal
                if v != 0 {
                    self.wireframe_indices.push(tl_corner);
                    self.wireframe_indices.push(tr_corner);
                }
            }
        }
    }
}
