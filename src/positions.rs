use nalgebra::Vector3;

use crate::Body;

pub const FIGURE_EIGHT: [Body; 3] = [
    Body {
        r: Vector3::new(0.0, 0.0, 0.0),
        v: Vector3::new(-0.93240737, -0.86473146, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(-0.97000436, 0.24308753, 0.0),
        v: Vector3::new(0.4662036850, 0.4323657300, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(0.97000436, -0.24308753, 0.0),
        v: Vector3::new(0.4662036850, 0.4323657300, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
];

pub const RANDOM: [Body; 3] = [
    Body {
        r: Vector3::new(0.3, 0.5, 0.6),
        v: Vector3::new(-0.1, -0.2, 0.4),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(-0.4, 0.2, 0.0),
        v: Vector3::new(0.6, 0.6, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(0.3, -0.4, 0.0),
        v: Vector3::new(0.9, 0.1, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
];
