use crate::features::tuples;
#[cfg(test)]
mod tests {
    use crate::features::tuples::EPSILON;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_struct_as_point() {
        let point = tuples::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 1.,
        };
        assert_eq!(point.w, 1.);
    }

    #[test]
    fn test_tuple_struct_as_vector() {
        let vector1 = tuples::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 0.,
        };
        assert_ne!(vector1.w, 1.);
    }

    #[test]
    fn test_point_fn() {
        let p1 = tuples::point(4., -4., 3.);
        assert_eq!(p1.x, 4.);
        assert_eq!(p1.y, -4.);
        assert_eq!(p1.z, 3.,);
        assert_eq!(p1.w, 1.);
    }

    #[test]
    fn test_vector_fn() {
        let v1 = tuples::vector(4., -4., 3.);
        assert_eq!(v1.x, 4.);
        assert_eq!(v1.y, -4.);
        assert_eq!(v1.z, 3.);
        assert_eq!(v1.w, 0.);
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
        assert_eq!(option.is_none(), true); // two points cannot be added
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
        assert_eq!(result.w, 1.);
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
        assert_eq!(result.w, 0.);
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
        assert_eq!(result.w, 1.);
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
        assert_eq!(result.w, 0.);
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
        assert_eq!(result.w, 0.);
    }

    #[test]
    fn test_tuple_negation() {
        let tuple = tuples::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: 4.,
        };
        let result = tuples::negate(tuple);
        assert_eq!(result.x, -1.);
        assert_eq!(result.y, 2.);
        assert_eq!(result.z, -3.);
        assert_eq!(result.w, -4.);
    }
    #[test]
    fn test_scalar_multiplication_v1() {
        let scalar = 3.5;
        let tuple = tuples::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: 4.,
        };
        let result = tuples::scalar(tuple, scalar);
        assert_eq!(result.x, 3.5);
        assert_eq!(result.y, -7.);
        assert_eq!(result.z, 10.5);
        assert_eq!(result.w, 14.);
    }

    #[test]
    fn test_scalar_multiplication_v2() {
        let scalar = 0.5;
        let tuple = tuples::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: 4.,
        };
        let result = tuples::scalar(tuple, scalar);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, -1.);
        assert_eq!(result.z, 1.5);
        assert_eq!(result.w, 2.);
    }

    #[test]
    fn test_compute_magnitude_v1() {
        let tuple = tuples::Tuple {
            x: 0.,
            y: 1.,
            z: 0.,
            w: 0.,
        };
        let magnitude = tuples::magnitude(tuple);
        assert_eq!(magnitude, 1.0_f64);
    }

    #[test]
    fn test_compute_magnitude_v2() {
        let tuple = tuples::Tuple {
            x: 0.,
            y: 0.,
            z: 1.,
            w: 0.,
        };
        let magnitude = tuples::magnitude(tuple);
        assert_eq!(magnitude, 1.0_f64);
    }

    #[test]
    fn test_compute_magnitude_v3() {
        let tuple = tuples::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 0.,
        };
        let magnitude = tuples::magnitude(tuple);
        // assert_eq!(magnitude, 14.0_f64.sqrt());  This still passes
        assert!(magnitude < 14_f64.sqrt() + EPSILON / 2.);
        assert!(magnitude > 14_f64.sqrt() - EPSILON / 2.);
    }

    #[test]
    fn test_compute_magnitude_v4() {
        let tuple = tuples::Tuple {
            x: -1.,
            y: -2.,
            z: -3.,
            w: 0.,
        };
        let magnitude = tuples::magnitude(tuple);
        // assert_eq!(magnitude, 14.0_f64.sqrt());  This still passes
        assert!(magnitude < 14_f64.sqrt() + EPSILON / 2.);
        assert!(magnitude > 14_f64.sqrt() - EPSILON / 2.);
    }

    #[test]
    fn test_normalize_v1() {
        let vector = tuples::vector(4., 0., 0.);
        let normalized_vector = tuples::normalize(vector);
        // assert_eq!(magnitude, 14.0_f64.sqrt());  This still passes
        assert_eq!(normalized_vector.x, 1.);
        assert_eq!(normalized_vector.y, 0.);
        assert_eq!(normalized_vector.z, 0.);
    }

    #[test]
    fn test_normalize_v2() {
        let vector = tuples::vector(1., 2., 3.); // Magnitude of vector = sqrt(14)
        let normalized_vector = tuples::normalize(vector);

        // all this mess is to approximately equal normalized x, y , z components d/t fp rounding
        assert!(
            (1. / 14.0_f64.sqrt() - EPSILON / 2. < normalized_vector.x)
                && (normalized_vector.x < 1. / 14.0_f64.sqrt() + EPSILON / 2.)
        );

        assert!(
            (2. / 14.0_f64.sqrt() - EPSILON / 2. < normalized_vector.y)
                && (normalized_vector.y < 2. / 14.0_f64.sqrt() + EPSILON / 2.)
        );

        assert!(
            (3. / 14.0_f64.sqrt() - EPSILON / 2. < normalized_vector.z)
                && (normalized_vector.z < 3. / 14.0_f64.sqrt() + EPSILON / 2.)
        );
    }

    #[test]
    fn test_dot_product() {
        let vector1 = tuples::vector(1., 2., 3.);
        let vector2 = tuples::vector(2., 3., 4.);

        let dot_product = tuples::dot_product(vector1, vector2);

        assert!((20.0 - EPSILON / 2. < dot_product) && (dot_product < 20.0 + EPSILON / 2.));
    }

    #[test]
    fn test_cross_product() {
        let vector1 = tuples::vector(1., 2., 3.);
        let vector2 = tuples::vector(2., 3., 4.);

        let cp_a = tuples::cross(vector1, vector2);
        assert!((-1. - EPSILON / 2. < cp_a.x) && (cp_a.x < -1. + EPSILON / 2.));
        assert!((2. - EPSILON / 2. < cp_a.y) && (cp_a.y < 2. + EPSILON / 2.));
        assert!((-1. - EPSILON / 2. < cp_a.z) && (cp_a.z < -1. + EPSILON / 2.));

        let cp_b = tuples::cross(vector2, vector1);
        assert!((1. - EPSILON / 2. < cp_b.x) && (cp_b.x < 1. + EPSILON / 2.));
        assert!((-2. - EPSILON / 2. < cp_b.y) && (cp_b.y < -2. + EPSILON / 2.));
        assert!((1. - EPSILON / 2. < cp_b.z) && (cp_b.z < 1. + EPSILON / 2.));
    }
}
