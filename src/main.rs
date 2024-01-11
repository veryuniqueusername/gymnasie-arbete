mod positions;

use std::f32::consts::PI;

use macroquad::prelude::*;
use nalgebra::Vector3;
use positions::*;

const DELTA_TIME: f64 = 0.0000005; // How much time passes between each calculation
const SIMULATION_TIME: f64 = 60.0; // How many (simulation) seconds to run each simulation for
const TARGET_FRAME_TIME: f64 = 1.0 / 30.0;
const PATH_LENGTH: usize = 512; // How many segments the path is made of
const PATH_SKIP: usize = 0; // How many frames skipped between a segment is added to the path
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

const FIXED_ZOOM: bool = true; // Whether the zoom amount should be fixed or if all objects should be visible at all times
const ZOOM: f32 = 5.0; // How far away the camera is, higher value = more zoomed out
const FOLLOW_COM: bool = true; // If the camera should follow center of mass or origin
const DRAW_ORBITS: bool = false; // If the path should be stored relative to the center of mass or absolute positions in world, does nothing if FOCUS_COM is false

#[derive(Debug)]
pub struct Body {
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
    let mut elapsed_time: f64 = 0.0;

    let mut bodies = RANDOM;

    let mut center_of_mass: Vec3 = Vec3::new(0., 0., 0.);
    if FOLLOW_COM {
        bodies.iter().for_each(|body| {
            let r: Vector3<f32> = nalgebra::convert(body.r);
            let r: Vec3 = r.into();
            center_of_mass += r
        });
        center_of_mass /= bodies.len() as f32;
    }

    // Initiate path
    let mut path: Vec<[Vec3; PATH_LENGTH]> = vec![[Vec3::ZERO; PATH_LENGTH]; bodies.len()];
    for i in 0..path.len() {
        let p: Vector3<f32> = nalgebra::convert(bodies[i].r);
        let p: Vec3 = p.into();
        let p: Vec3 = p - if DRAW_ORBITS {
            center_of_mass
        } else {
            Vec3::ZERO
        };

        for j in 0..PATH_LENGTH {
            path[i][j] = p;
        }
    }

    let mut frames_since_last_segment: usize = 0; // How many frames have passed since the last path segment was added

    loop {
        clear_background(Color::from_hex(0x1a1223)); // Background

        set_default_camera();

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

            if get_time() - iteration_start_time > TARGET_FRAME_TIME {
                break 'calculations;
            }
        }

        set_camera(&Camera3D {
            position: Vec3::new(
                ZOOM * (elapsed_time as f32 / 10.).cos(),
                ZOOM * (elapsed_time as f32 / 10.).sin(),
                ZOOM + ZOOM * (elapsed_time as f32 / 10.).sin(),
            ) + if FOLLOW_COM {
                center_of_mass
            } else {
                Vec3::ZERO
            },
            target: if FOLLOW_COM {
                center_of_mass
            } else {
                Vec3::new(0., 0., 0.)
            },
            up: Vec3::new(0., 0., 1.),
            ..Default::default()
        });

        center_of_mass = Vec3::new(0., 0., 0.);
        bodies.iter().for_each(|body: &Body| {
            let r: Vector3<f32> = nalgebra::convert(body.r);
            let r: Vec3 = r.into();
            center_of_mass += r
        });
        center_of_mass /= bodies.len() as f32;

        draw_sphere(Vec3::new(0., 0., 0.), 0.05, None, WHITE);
        draw_sphere(center_of_mass, 0.05, None, WHITE);

        three_body::draw_grid_ex(
            500,
            1.0,
            WHITE,
            Color::new(0.5, 0.5, 0.5, 0.5),
            Vec3::ZERO,
            Quat::from_rotation_x(PI / 2.0),
        );

        if frames_since_last_segment == PATH_SKIP {
            frames_since_last_segment = 0;
        } else {
            frames_since_last_segment += 1;
        }

        for i in 0..bodies.len() {
            let r: Vector3<f32> = nalgebra::convert(bodies[i].r);
            let r: Vec3 = r.into();
            let v: Vector3<f32> = nalgebra::convert(bodies[i].v);
            let v: Vec3 = v.into();
            let a: Vector3<f32> = nalgebra::convert(bodies[i].a);
            let a: Vec3 = a.into();

            // Draw dot
            draw_sphere(r, 0.1, None, COLORS[i]);

            // Add segment to path
            if frames_since_last_segment == 0 {
                path[i].rotate_right(1);
                path[i][0] = r - if DRAW_ORBITS {
                    center_of_mass
                } else {
                    Vec3::ZERO
                };
            }
            // Draw path
            for j in 1..path[i].len() {
                let c = COLORS[i];
                let c = Color::new(c.r, c.g, c.b, c.a - (j as f32 / PATH_LENGTH as f32));
                draw_line_3d(
                    path[i][j]
                        + if DRAW_ORBITS {
                            center_of_mass
                        } else {
                            Vec3::ZERO
                        },
                    path[i][j - 1]
                        + if DRAW_ORBITS {
                            center_of_mass
                        } else {
                            Vec3::ZERO
                        },
                    c,
                );
            }

            // Draw velocity and acceleration vectors
            if DRAW_VELOCITY {
                draw_line_3d(r, r + v, VELOCITY_COLOR);
            }
            if DRAW_ACCELERATION {
                draw_line_3d(r, r + a, ACCELERATION_COLOR);
            }
        }

        next_frame().await
    }
}
