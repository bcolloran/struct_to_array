use proptest::prelude::*;
use struct_to_array::{StructToArray, StructToVec};

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct Point2D {
    x: f32,
    y: f32,
}

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct Tuple4(i32, i32, i32, i32);

proptest! {
    #[test]
    fn prop_to_vec_from_vec_roundtrip_point2d(x: f32, y: f32) {
        // Filter out NaN values
        prop_assume!(!x.is_nan() && !y.is_nan());

        let original = Point2D { x, y };
        let vec = original.clone().to_vec();
        let reconstructed = Point2D::from_vec(&vec);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_to_vec_from_vec_roundtrip_point3d(x: f64, y: f64, z: f64) {
        // Filter out NaN values
        prop_assume!(!x.is_nan() && !y.is_nan() && !z.is_nan());

        let original = Point3D { x, y, z };
        let vec = original.clone().to_vec();
        let reconstructed = Point3D::from_vec(&vec);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_to_vec_from_vec_roundtrip_tuple4(a: i32, b: i32, c: i32, d: i32) {
        let original = Tuple4(a, b, c, d);
        let vec = original.clone().to_vec();
        let reconstructed = Tuple4::from_vec(&vec);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_to_vec_length_matches_num_fields(x: f32, y: f32) {
        prop_assume!(!x.is_nan() && !y.is_nan());

        let point = Point2D { x, y };
        let vec = point.to_vec();

        prop_assert_eq!(vec.len(), Point2D::num_fields());
    }

    #[test]
    fn prop_to_vec_elements_match_fields(x: f32, y: f32) {
        prop_assume!(!x.is_nan() && !y.is_nan());

        let point = Point2D { x, y };
        let vec = point.to_vec();

        prop_assert_eq!(vec[0], x);
        prop_assert_eq!(vec[1], y);
    }

    #[test]
    fn prop_to_vec_equals_to_arr_to_vec(x: f64, y: f64, z: f64) {
        prop_assume!(!x.is_nan() && !y.is_nan() && !z.is_nan());

        let point = Point3D { x, y, z };

        // to_vec() should produce the same result as to_arr().to_vec()
        let vec_direct = point.clone().to_vec();
        let vec_via_arr = point.to_arr().to_vec();

        prop_assert_eq!(vec_direct, vec_via_arr);
    }
}

// Note: Testing panic behavior with proptest doesn't work well because proptest
// tries to shrink failures, which conflicts with expected panics.
// These tests are now in integration.rs and regression_specific_cases.rs instead.
