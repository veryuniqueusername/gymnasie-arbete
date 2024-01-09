mod positions;

use macroquad::prelude::*;
use nalgebra::Vector3;
use positions::*;

const DELTA_TIME: f64 = 0.000005; // How much time passes between each calculation
const SIMULATION_TIME: f64 = 60.0; // How many (simulation) seconds to run each simulation for
const FRAME_TIME: f64 = 1.0 / 60.0; // Seconds per frame
const SCALE: f32 = 70.0; // How many times the zoom is
const PATH_LENGTH: usize = 512; // How many segments the path is made of
const PATH_SKIP: usize = 1; // How many frames pass until a segment is added to the path
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

const DRAW_VELOCITY: bool = true;
const DRAW_ACCELERATION: bool = true;

const FOCUS_COM: bool = true; // If the camera should be centered on center of mass or origin
const DRAW_PATH_RELATIVE_TO_COM: bool = true; // If the path is drawn to be relative to the center of mass or origin, does nothing if FOCUS_COM is false

#[derive(Debug)]
struct Body {
    r: Vector3<f64>,
    v: Vector3<f64>,
    a: Vector3<f64>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simulation".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut elapsed_time = 0.0;

    let mut bodies = RANDOM;

    let mut center_of_mass: Vector3<f64> = Vector3::zeros();
    if FOCUS_COM {
        bodies.iter().for_each(|body| center_of_mass += body.r);
        center_of_mass /= bodies.len() as f64;
    }

    let mut path: Vec<[Vector3<f32>; PATH_LENGTH]> =
        vec![[Vector3::zeros(); PATH_LENGTH]; bodies.len()];
    for i in 0..path.len() {
        let p = Vector3::new(
            screen_width() / 2.0
                + (bodies[i].r.x
                    - if DRAW_PATH_RELATIVE_TO_COM {
                        center_of_mass.x
                    } else {
                        0.0
                    }) as f32
                    * SCALE,
            screen_height() / 2.0
                - (bodies[i].r.y
                    - if DRAW_PATH_RELATIVE_TO_COM {
                        center_of_mass.y
                    } else {
                        0.0
                    }) as f32
                    * SCALE,
            bodies[i].r.z as f32,
        );
        for j in 0..PATH_LENGTH {
            path[i][j] = p;
        }
    }

    let mut frames_since_last_segment: usize = 0; // How many frames have passed since the last path segment was added

    loop {
        clear_background(Color::from_hex(0x1a1223)); // Background
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

        let center_width = screen_width() / 2.0;
        let center_height = screen_height() / 2.0;

        center_of_mass = Vector3::zeros();
        bodies.iter().for_each(|body| center_of_mass += body.r);
        center_of_mass /= bodies.len() as f64;

        draw_circle(
            center_width
                + if FOCUS_COM {
                    0.0
                } else {
                    SCALE * (center_of_mass.x as f32)
                },
            center_height
                - if FOCUS_COM {
                    0.0
                } else {
                    SCALE * (center_of_mass.y as f32)
                },
            2.0,
            WHITE,
        ); // Center of mass

        if !FOCUS_COM {
            center_of_mass = Vector3::zeros(); // Simple way to shift focus on origin, same as ignoring CoM
        }

        draw_circle(
            center_width - SCALE * (center_of_mass.x as f32),
            center_height + SCALE * (center_of_mass.y as f32),
            2.0,
            WHITE,
        ); // Center

        // Grid
        for i in (-(screen_width() / SCALE) as i32)..=((screen_width() / SCALE) as i32) {
            draw_line(
                center_width + SCALE * (i as f32 - (center_of_mass.x % 1.0) as f32),
                0.0,
                center_width + SCALE * (i as f32 - (center_of_mass.x % 1.0) as f32),
                screen_height(),
                0.5,
                GRAY,
            )
        }
        for i in (-(screen_height() / SCALE) as i32)..=((screen_height() / SCALE) as i32) {
            draw_line(
                0.0,
                center_height - SCALE * (i as f32 - (center_of_mass.y % 1.0) as f32),
                screen_width(),
                center_height - SCALE * (i as f32 - (center_of_mass.y % 1.0) as f32),
                0.5,
                GRAY,
            )
        }

        if frames_since_last_segment == PATH_SKIP {
            frames_since_last_segment = 0;
        } else {
            frames_since_last_segment += 1;
        }

        for i in 0..bodies.len() {
            // Draw dot

            let pos_x: f32 = center_width + SCALE * (bodies[i].r.x - center_of_mass.x) as f32;
            let pos_y: f32 = center_height - SCALE * (bodies[i].r.y - center_of_mass.y) as f32;
            draw_circle(pos_x, pos_y, 5.0, COLORS[i]);

            // Add segment to path
            if frames_since_last_segment == 0 {
                path[i].rotate_right(1);
                path[i][0] = Vector3::new(
                    pos_x
                        + if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.x as f32
                        }, // Revert to being relative to origin instead of CoM
                    pos_y
                        - if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.y as f32
                        }, // Revert to being relative to origin instead of CoM
                    0.0,
                );
            }
            // Draw path
            for j in 1..path[i].len() {
                let c = COLORS[i];
                let c = Color::new(c.r, c.g, c.b, c.a - (j as f32 / PATH_LENGTH as f32));
                draw_line(
                    path[i][j - 1].x
                        - if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.x as f32
                        },
                    path[i][j - 1].y
                        + if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.y as f32
                        },
                    path[i][j].x
                        - if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.x as f32
                        },
                    path[i][j].y
                        + if DRAW_PATH_RELATIVE_TO_COM {
                            0.0
                        } else {
                            SCALE * center_of_mass.y as f32
                        },
                    2.0,
                    c,
                );
            }

            // Draw velocity and acceleration vectors
            if DRAW_VELOCITY {
                draw_line(
                    pos_x,
                    pos_y,
                    pos_x + (bodies[i].v.x as f32 * SCALE),
                    pos_y - (bodies[i].v.y as f32 * SCALE),
                    2.0,
                    VELOCITY_COLOR,
                );
            }
            if DRAW_ACCELERATION {
                draw_line(
                    pos_x,
                    pos_y,
                    pos_x + (bodies[i].a.x as f32 * SCALE),
                    pos_y - (bodies[i].a.y as f32 * SCALE),
                    2.0,
                    ACCELERATION_COLOR,
                );
            }
        }

        next_frame().await
    }
}
