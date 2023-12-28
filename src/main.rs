use macroquad::prelude::*;
use nalgebra::Vector3;

const DELTA_TIME: f64 = 0.001;
const SIMULATION_TIME: f64 = 60.0;
const FRAMERATE: f64 = 60.0;
const SCALE: f64 = 200.0;
const COLORS: [macroquad::color::Color; 3] = [
    Color::new(1.0, 0.5, 0.5, 1.0),
    Color::new(0.5, 1.0, 0.5, 1.0),
    Color::new(0.5, 0.5, 1.0, 1.0),
];

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

    let mut bodies = [
        Body {
            r: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(-0.93240737, -0.86473146, 0.0),
            a: Vector3::zeros(),
        },
        Body {
            r: Vector3::new(-0.97000436, 0.24308753, 0.0),
            v: Vector3::new(0.4662036850, 0.4323657300, 0.0),
            a: Vector3::zeros(),
        },
        Body {
            r: Vector3::new(0.97000436, -0.24308753, 0.0),
            v: Vector3::new(0.4662036850, 0.4323657300, 0.0),
            a: Vector3::zeros(),
        },
    ];

    loop {
        clear_background(Color::from_hex(0x1a1223)); // Background
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 2.0, WHITE); // Center
        for i in (-(screen_width() / SCALE as f32) as i32 / 2)
            ..=((screen_width() / SCALE as f32) as i32 / 2)
        {
            draw_line(
                screen_width() / 2.0 + SCALE as f32 * i as f32,
                0.0,
                screen_width() / 2.0 + SCALE as f32 * i as f32,
                screen_height(),
                0.5,
                GRAY,
            )
        }
        for i in (-(screen_height() / SCALE as f32) as i32 / 2)
            ..=((screen_height() / SCALE as f32) as i32 / 2)
        {
            draw_line(
                0.0,
                screen_height() / 2.0 + SCALE as f32 * i as f32,
                screen_width(),
                screen_height() / 2.0 + SCALE as f32 * i as f32,
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

            // Update velocity
            for body in bodies.iter_mut() {
                body.v += body.a * DELTA_TIME;
            }

            // Update position
            for body in bodies.iter_mut() {
                body.r += body.v * DELTA_TIME;
            }

            elapsed_time += DELTA_TIME;

            if get_time() - iteration_start_time > 1.0 / FRAMERATE {
                break 'calculations;
            }
        }

        for i in 0..bodies.len() {
            let pos_x: f32 = (screen_width() / 2.0) + ((bodies[i].r.x * SCALE) as f32);
            let pos_y: f32 = (screen_height() / 2.0) - ((bodies[i].r.y * SCALE) as f32);
            draw_circle(pos_x, pos_y, 5.0, COLORS[i]);
            draw_line(
                pos_x,
                pos_y,
                pos_x + ((bodies[i].v.x * SCALE) as f32),
                pos_y - ((bodies[i].v.y * SCALE) as f32),
                2.0,
                RED,
            );
            draw_line(
                pos_x,
                pos_y,
                pos_x + ((bodies[i].a.x * SCALE) as f32),
                pos_y - ((bodies[i].a.y * SCALE) as f32),
                2.0,
                YELLOW,
            );
        }

        next_frame().await
    }
}
