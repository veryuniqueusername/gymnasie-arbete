use macroquad::prelude::*;

/// Draw a rotated grid centered at a specified point
pub fn draw_grid_ex(
    slices: u32,
    spacing: f32,
    axes_color: Color,
    other_color: Color,
    center: Vec3,
    rotation: Quat,
) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..half_slices + 1 {
        let color = if i == 0 { axes_color } else { other_color };

        let start = vec3(i as f32 * spacing, 0., -half_slices as f32 * spacing);
        let end = vec3(i as f32 * spacing, 0., half_slices as f32 * spacing);

        draw_line_3d(
            rotation.mul_vec3(start) + center,
            rotation.mul_vec3(end) + center,
            color,
        );

        let start = vec3(-half_slices as f32 * spacing, 0., i as f32 * spacing);
        let end = vec3(half_slices as f32 * spacing, 0., i as f32 * spacing);

        draw_line_3d(
            rotation.mul_vec3(start) + center,
            rotation.mul_vec3(end) + center,
            color,
        );
    }
}
