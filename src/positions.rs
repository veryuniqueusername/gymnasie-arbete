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

pub const TWO_BODY: [Body; 2] = [
    Body {
        r: Vector3::new(-1.0, 0.0, 0.0),
        v: Vector3::new(0.3, -0.3, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(1.0, 0.0, 0.0),
        v: Vector3::new(0.7, 0.3, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
];

pub const FOUR_BODY_SQUARE: [Body; 4] = [
    Body {
        r: Vector3::new(1.0, 1.0, 0.0),
        v: Vector3::new(-0.8, 0.0, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(1.0, -1.0, 0.0),
        v: Vector3::new(0.0, 0.8, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(-1.0, -1.0, 0.0),
        v: Vector3::new(0.8, 0.0, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(-1.0, 1.0, 0.0),
        v: Vector3::new(0.0, -0.8, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
];

pub const RANDOM: [Body; 3] = [
    Body {
        r: Vector3::new(-2.0, 0.0, 0.0),
        v: Vector3::new(0.0, -0.5, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(1.0, 0.0, 0.0),
        v: Vector3::new(0.0, 1.0, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
    Body {
        r: Vector3::new(0.0, 0.0, 0.0),
        v: Vector3::new(0.0, 0.0, 0.0),
        a: Vector3::new(0.0, 0.0, 0.0),
    },
];
