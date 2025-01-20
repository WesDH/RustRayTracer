mod features {
    pub mod tuples;
}
mod chapters {
    pub mod ch_01_tuples_points_vectors;
}

mod testing {
    pub mod tuples_test;
}

// Operations on vectors and points:
//fn add_tuple(a1: Struct, a2: Point) -> Point {}

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
    chapters::ch_01_tuples_points_vectors::run();
    // let p1 = point(0.1,0.2, 0.3);
    // let v1 = vector(0.1,0.2, 0.3);
}
