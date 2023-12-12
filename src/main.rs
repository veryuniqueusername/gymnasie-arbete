use nalgebra::Vector3;
use three_body::run;

const DELTA_TIME: f64 = 0.001;
const SIMULATION_TIME: f64 = 60.0;

fn main() {
    run();
}

fn _main() {
    let mut positions: Vec<(Vector3<f64>, Vector3<f64>, Vector3<f64>)> = Vec::new();

    // Mass
    const M_A: f64 = 1.0;
    const M_B: f64 = 1.0;
    const M_C: f64 = 1.0;

    // Positions
    let mut r_a: Vector3<f64> = Vector3::new(1.0, 0.0, 0.0);
    let mut r_b: Vector3<f64> = Vector3::new(-0.5, 0.8660254038, 0.0);
    let mut r_c: Vector3<f64> = Vector3::new(-0.5, -0.8660254038, 0.0);

    // Momenta
    let mut p_a: Vector3<f64> = Vector3::new(0.0, 0.5, 0.0);
    let mut p_b: Vector3<f64> = Vector3::new(-0.4330127019, -0.25, 0.0);
    let mut p_c: Vector3<f64> = Vector3::new(0.4330127019, -0.25, 0.0);

    // Force vectors, f_ab means the force A has on B; A pulls B with the force f_ab
    // f_ba = -f_ab
    let mut f_ab: Vector3<f64>;
    let mut f_ac: Vector3<f64>;
    let mut f_bc: Vector3<f64>;

    let mut time = 0.0;

    while time < SIMULATION_TIME {
        // Calculate the new forces and assign them to their variables
        f_ab = calculate_gravitational_force(M_A, r_a, M_B, r_b);
        f_ac = calculate_gravitational_force(M_A, r_a, M_C, r_c);
        f_bc = calculate_gravitational_force(M_B, r_b, M_C, r_c);

        // Update momenta
        p_a += ((-f_ab) + -(f_ac)) * DELTA_TIME;
        p_b += ((f_ab) + -(f_bc)) * DELTA_TIME;
        p_c += ((f_ac) + (f_bc)) * DELTA_TIME;

        // Update positions
        r_a += p_a / M_A * DELTA_TIME;
        r_b += p_b / M_B * DELTA_TIME;
        r_c += p_c / M_C * DELTA_TIME;

        time += DELTA_TIME;

        positions.push((r_a, r_b, r_c));
    }

    println!("{:?}", positions);
}

// Outputs the force that 1 pulls on 2
fn calculate_gravitational_force(
    m_1: f64,
    r_1: Vector3<f64>,
    m_2: f64,
    r_2: Vector3<f64>,
) -> Vector3<f64> {
    // F = G * m_1 * m_2 * r_hat / r^2
    // G = 6.67430 * 10^-11
    const G: f64 = 0.0000000000667430;
    let r: Vector3<f64> = r_1 - r_2;
    let r_magnitude = r.x.powi(2) + r.y.powi(2) + r.z.powi(2);
    let r_hat: Vector3<f64> = r / r_magnitude;

    let f = G * m_1 * m_2 / r_magnitude.powi(2);

    f * r_hat
}
