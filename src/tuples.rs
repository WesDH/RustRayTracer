pub const EPSILON: f64 = 0.00001; // f64 margin of error, used for f64 equality compares

pub struct Tuple {
    // w == 1 for a point, w == 0 for a vector
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: i8,
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    let point = Tuple { x, y, z, w: 1 };
    point
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    let vector = Tuple { x, y, z, w: 0 };
    vector
}

pub fn equals_fl(a: f64, b: f64) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        false
    }
}

pub fn is_eq_point(p1: Tuple, p2: Tuple) -> bool {
    // This function compares two points are equal within the margin EPSILON
    if (p1.x - p2.x).abs() < EPSILON
        && (p1.y - p2.y).abs() < EPSILON
        && (p1.z - p2.z).abs() < EPSILON
        && (p1.w + p2.w) == 2
    // w=1 by definition of a point. p1.w + p2.w should equal 2
    {
        true
    } else {
        false
    }
}

pub fn is_eq_vector(v1: Tuple, v2: Tuple) -> bool {
    // This function compares two vectors are equal within the margin EPSILON
    if (v1.x - v2.x).abs() < EPSILON
        && (v1.y - v2.y).abs() < EPSILON
        && (v1.z - v2.z).abs() < EPSILON
        && (v1.w + v2.w) == 0
    // w=0 by definition of a vector. v1.w + v2.w should equal 0
    {
        true
    } else {
        false
    }
}

pub fn add(t1: Tuple, t2: Tuple) -> Option<Tuple> {
    // Adds two tuples, note the following 3 cases for w value:
    // 1) vec + point = point_result
    // 2) vec + vec = vec_result
    // 3) point + point does not make sense: p1.w + p2.w == 2
    if t1.w + t2.w > 1 {
        eprint!("w component result for {} and {} when added sum to {} ", t1.w, t2.w, t1.w + t2.w);
        eprint!("Note that two points cannot be added together.");
        None
    } else {
        let tuple = Tuple {
            x: t1.x + t2.x,
            y: t1.y + t2.y,
            z: t1.z + t2.z,
            w: t1.w + t2.w,
        };
        Some(tuple)
    }
}

pub fn subtract(t1: Tuple, t2: Tuple) -> Option<Tuple> {
    // Subtracts t1 from t2,
    // recalling that subtraction is not communicative (operand order is crucial).
    // Note the following 3 cases for w value:
    // 1) vec - point does not make sense: v1.w - p2.w == -1
    // 2) vec - vec = vec  w->0
    // 3) point - vector == point  w->1
    if t1.w - t2.w < 0 {
        eprint!("w component res for {} and {} when subtracted equals {}", t1.w, t2.w, t1.w - t2.w);
        eprint!("Note that a point cannot be subtracted from a vector.");
        None
    } else {
        let tuple = Tuple {
            x: t1.x - t2.x,
            y: t1.y - t2.y,
            z: t1.z - t2.z,
            w: t1.w - t2.w,
        };
        Some(tuple)
    }
}

