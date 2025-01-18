pub use crate::tuples;
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_struct_as_point() {
        let point = tuples::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 1,
        };
        assert_eq!(point.w, 1);
    }

    #[test]
    fn test_tuple_struct_as_vector() {
        let vector1 = tuples::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 0,
        };
        assert_ne!(vector1.w, 1);
    }

    #[test]
    fn test_point_fn() {
        let p1 = tuples::point(4., -4., 3.);
        assert_eq!(p1.x, 4.);
        assert_eq!(p1.y, -4.);
        assert_eq!(p1.z, 3.,);
        assert_eq!(p1.w, 1);
    }

    #[test]
    fn test_vector_fn() {
        let v1 = tuples::vector(4., -4., 3.);
        assert_eq!(v1.x, 4.);
        assert_eq!(v1.y, -4.);
        assert_eq!(v1.z, 3.);
        assert_eq!(v1.w, 0);
    }

    #[test]
    fn test1_equals_fl_fn() {
        let offset = tuples::EPSILON + 0.000001; // outside of bounds set by EPSILON
        let v1 = tuples::vector(4., 0., 0.);
        let v2 = tuples::vector(4. + offset, 0., 0.);
        assert_eq!(tuples::equals_fl(v1.x, v2.x), false);
    }

    #[test]
    fn test2_equals_fl_fn() {
        let offset = tuples::EPSILON - 0.000001; // inside of bounds set by EPSILON
        let v1 = tuples::vector(4., 0., 0.);
        let v2 = tuples::vector(4. + offset, 0., 0.);
        assert_eq!(tuples::equals_fl(v1.x, v2.x), true);
    }

    #[test]
    fn test1_is_eq_point_fn() {
        let offset = tuples::EPSILON - 0.000001; // inside of bounds set by EPSILON
        let v1 = tuples::point(4., 0., 0.);
        let v2 = tuples::point(4., 0. + offset, 0.);
        assert_eq!(tuples::is_eq_point(v1, v2), true);
    }

    #[test]
    fn test2_is_eq_point_fn() {
        let offset = tuples::EPSILON + 0.000001; // outside of bounds set by EPSILON
        let v1 = tuples::point(4., 0., 0.);
        let v2 = tuples::point(4., 0. + offset, 0.);
        assert_eq!(tuples::is_eq_point(v1, v2), false);
    }

    #[test]
    fn test1_is_eq_vector() {
        let offset = tuples::EPSILON - 0.000001; // inside of bounds set by EPSILON
        let v1 = tuples::vector(4., 0., 0.);
        let v2 = tuples::vector(4., 0. + offset, 0.);
        assert_eq!(tuples::is_eq_vector(v1, v2), true);
    }

    #[test]
    fn test2_is_eq_vector() {
        let offset = tuples::EPSILON + 0.000001; // outside of bounds set by EPSILON
        let v1 = tuples::vector(4., 0., 0.);
        let v2 = tuples::vector(4., 0. + offset, 0.);
        assert_eq!(tuples::is_eq_vector(v1, v2), false);
    }

    #[test]
    fn test_add_tuples_p_and_p() {
        let p1 = tuples::point(3., -2., 5.);
        let p2 = tuples::point(-2., 3., 1.);
        let option = tuples::add(p1, p2);
        assert_eq!(option.is_none(), true);  // two points cannot be added
    }

    #[test]
    fn test_add_tuples_v_and_p() {
        let v1 = tuples::vector(3., -2., 5.);
        let p1 = tuples::point(-2., 3., 1.);
        let option = tuples::add(v1, p1);
        assert_eq!(option.is_some(), true);
        let result = option.unwrap();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
        assert_eq!(result.z, 6.);
        assert_eq!(result.w, 1);
    }

    #[test]
    fn test_add_tuples_v_and_v() {
        let v1 = tuples::vector(3., -2., 5.);
        let p1 = tuples::vector(-2., 3., 1.);
        let option = tuples::add(v1, p1);
        assert_eq!(option.is_some(), true);
        let result = option.unwrap();
        assert_eq!(result.x, 1.);
        assert_eq!(result.y, 1.);
        assert_eq!(result.z, 6.);
        assert_eq!(result.w, 0);
    }

    #[test]
    fn test_subtract_tuples_v_and_p() {
        let vector = tuples::vector(3., 2., 1.);
        let point = tuples::point(5., 6., 7.);
        let option = tuples::subtract(vector, point);
        assert_eq!(option.is_none(), true);
    }

    #[test]
    fn test_subtract_tuples_p_and_v() {
        let point = tuples::point(3.5, 2., 1.);
        let vector = tuples::vector(5., 6., 7.);
        let option = tuples::subtract(point, vector);
        assert_eq!(option.is_some(), true);
        let result = option.unwrap();
        assert_eq!(result.x, -1.5);
        assert_eq!(result.y, -4.);
        assert_eq!(result.z, -6.);
        assert_eq!(result.w, 1);
    }

    #[test]
    fn test_subtract_tuples_p_and_p() {
        let p1 = tuples::point(3., 2., 1.);
        let p2 = tuples::point(5., 6., 7.);
        let option = tuples::subtract(p1, p2);
        assert_eq!(option.is_some(), true);
        let result = option.unwrap();
        assert_eq!(result.x, -2.);
        assert_eq!(result.y, -4.);
        assert_eq!(result.z, -6.);
        assert_eq!(result.w, 0);
    }

    #[test]
    fn test_subtract_tuples_v_and_v() {
        let v1 = tuples::vector(3., 3., 2.);
        let v2 = tuples::vector(5., 6., 7.);
        let option = tuples::subtract(v1, v2);
        assert_eq!(option.is_some(), true);
        let result = option.unwrap();
        assert_eq!(result.x, -2.);
        assert_eq!(result.y, -3.);
        assert_eq!(result.z, -5.);
        assert_eq!(result.w, 0);
    }
}
