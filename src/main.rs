mod positions;

use macroquad::prelude::*;
use nalgebra::Vector3;
use positions::*;

const DELTA_TIME: f64 = 0.000004; // How much time passes between each calculation
const SIMULATION_TIME: f64 = 60.0; // How many (simulation) seconds to run each simulation for
const FRAME_TIME: f64 = 1.0 / 60.0; // Seconds per frame
const SCALE: f32 = 50.0; // How many times the zoom is
const PATH_LENGTH: usize = 128; // How many segments the path is made of
const PATH_SKIP: usize = 2; // How many frames pass until a segment is added to the path
const COLORS: [macroquad::color::Color; 6] = [
    Color::new(1.0, 0.5, 0.5, 1.0),
    Color::new(0.5, 1.0, 0.5, 1.0),
    Color::new(0.5, 0.5, 1.0, 1.0),
    Color::new(1.0, 1.0, 0.5, 1.0),
    Color::new(1.0, 0.5, 1.0, 1.0),
    Color::new(0.5, 1.0, 1.0, 1.0),
];
const VELOCITY_COLOR: Color = Color::new(1.0, 0.0, 0.0, 0.5);
const ACCELERATION_COLOR: Color = Color::new(1.0, 1.0, 0.0, 0.5);

#[derive(Debug)]
struct Body {
    r: Vector3<f64>,
    v: Vector3<f64>,
    a: Vector3<f64>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simulation".to_owned(),
        window_width: 1200,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut elapsed_time = 0.0;

    let mut bodies = RANDOM;

    let mut path: Vec<[Vector3<f32>; PATH_LENGTH]> =
        vec![[Vector3::zeros(); PATH_LENGTH]; bodies.len()];
    for i in 0..path.len() {
        for j in 0..PATH_LENGTH {
            path[i][j] = Vector3::new(
                screen_width() / 2.0 + bodies[i].r.x as f32 * SCALE,
                screen_height() / 2.0 - bodies[i].r.y as f32 * SCALE,
                bodies[i].r.z as f32,
            );
        }
    }
    println!("{:#?}", path);
    let mut frames_since_last_segment: usize = 0;

    loop {
        let center_width = screen_width() / 2.0;
        let center_height = screen_height() / 2.0;

        clear_background(Color::from_hex(0x1a1223)); // Background
        draw_circle(center_width, center_height, 2.0, WHITE); // Center
        for i in (-(screen_width() / SCALE) as i32 / 2)..=((screen_width() / SCALE) as i32 / 2) {
            draw_line(
                center_width + SCALE * i as f32,
                0.0,
                center_width + SCALE * i as f32,
                screen_height(),
                0.5,
                GRAY,
            )
        }
        for i in (-(screen_height() / SCALE) as i32 / 2)..=((screen_height() / SCALE) as i32 / 2) {
            draw_line(
                0.0,
                center_height + SCALE * i as f32,
                screen_width(),
                center_height + SCALE * i as f32,
                0.5,
                GRAY,
            )
        }
        draw_text(&get_fps().to_string(), 5.0, 20.0, 20.0, WHITE); // FPS
        draw_text(&elapsed_time.to_string(), 5.0, 40.0, 20.0, WHITE); // Elapsed time

        let iteration_start_time = get_time();
        'calculations: loop {
            for body in bodies.iter_mut() {
                body.a = Vector3::zeros();
            }

            for i in 0..bodies.len() {
                for j in (i + 1)..bodies.len() {
                    // Calculate gravitational force with direction from i to j
                    let r: Vector3<f64> = bodies[j].r - bodies[i].r;
                    let f: Vector3<f64> = r / r.magnitude().powi(3);
                    bodies[i].a += f;
                    bodies[j].a -= f;
                }
            }

            // Update velocity and position
            for body in bodies.iter_mut() {
                body.v += body.a * DELTA_TIME;
                body.r += body.v * DELTA_TIME;
            }

            elapsed_time += DELTA_TIME;

            if get_time() - iteration_start_time > FRAME_TIME {
                break 'calculations;
            }
        }

        if frames_since_last_segment == PATH_SKIP {
            frames_since_last_segment = 0;
        } else {
            frames_since_last_segment += 1;
        }

        for i in 0..bodies.len() {
            // Draw dot

            let pos_x: f32 = center_width + (bodies[i].r.x as f32 * SCALE);
            let pos_y: f32 = center_height - (bodies[i].r.y as f32 * SCALE);
            draw_circle(pos_x, pos_y, 5.0, COLORS[i]);

            // Add segment to path
            if frames_since_last_segment == 0 {
                path[i].rotate_right(1);
                path[i][0] = Vector3::new(pos_x, pos_y, 0.0);
            }
            // Draw path
            for j in 1..path[i].len() {
                let c = COLORS[i];
                let c = Color::new(c.r, c.g, c.b, c.a - (j as f32 / PATH_LENGTH as f32));
                draw_line(
                    path[i][j - 1].x,
                    path[i][j - 1].y,
                    path[i][j].x,
                    path[i][j].y,
                    2.0,
                    c,
                );
            }

            // Draw velocity and acceleration vectors
            draw_line(
                pos_x,
                pos_y,
                pos_x + (bodies[i].v.x as f32 * SCALE),
                pos_y - (bodies[i].v.y as f32 * SCALE),
                2.0,
                VELOCITY_COLOR,
            );
            draw_line(
                pos_x,
                pos_y,
                pos_x + (bodies[i].a.x as f32 * SCALE),
                pos_y - (bodies[i].a.y as f32 * SCALE),
                2.0,
                ACCELERATION_COLOR,
            );
        }

        next_frame().await
    }
}
