use proptest::prelude::*;
use struct_to_array::StructToArray;

/// Property test: to_arr followed by from_arr should be identity
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
struct Tuple2(i32, i32);

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct Tuple5(u64, u64, u64, u64, u64);

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct GenericPair<T> {
    first: T,
    second: T,
}

proptest! {
    #[test]
    fn prop_point2d_roundtrip(x: f32, y: f32) {
        // Filter out NaN values since NaN != NaN
        prop_assume!(!x.is_nan() && !y.is_nan());

        let original = Point2D { x, y };
        let arr = original.clone().to_arr();
        let reconstructed = Point2D::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_point3d_roundtrip(x: f64, y: f64, z: f64) {
        // Filter out NaN values since NaN != NaN
        prop_assume!(!x.is_nan() && !y.is_nan() && !z.is_nan());

        let original = Point3D { x, y, z };
        let arr = original.clone().to_arr();
        let reconstructed = Point3D::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_tuple2_roundtrip(a: i32, b: i32) {
        let original = Tuple2(a, b);
        let arr = original.clone().to_arr();
        let reconstructed = Tuple2::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_tuple5_roundtrip(a: u64, b: u64, c: u64, d: u64, e: u64) {
        let original = Tuple5(a, b, c, d, e);
        let arr = original.clone().to_arr();
        let reconstructed = Tuple5::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_generic_pair_i32_roundtrip(first: i32, second: i32) {
        let original = GenericPair { first, second };
        let arr = original.clone().to_arr();
        let reconstructed = GenericPair::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_generic_pair_u64_roundtrip(first: u64, second: u64) {
        let original = GenericPair { first, second };
        let arr = original.clone().to_arr();
        let reconstructed = GenericPair::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }

    #[test]
    fn prop_array_elements_match_struct_fields(x: f32, y: f32) {
        // Filter out NaN values
        prop_assume!(!x.is_nan() && !y.is_nan());

        let point = Point2D { x, y };
        let arr = point.to_arr();

        prop_assert_eq!(arr[0], x);
        prop_assert_eq!(arr[1], y);
    }

    #[test]
    fn prop_from_arr_creates_correct_struct(a: i32, b: i32) {
        let arr = [a, b];
        let tuple = Tuple2::from_arr(arr);

        prop_assert_eq!(tuple.0, a);
        prop_assert_eq!(tuple.1, b);
    }

    #[test]
    fn prop_num_fields_is_constant(x: f32, y: f32) {
        prop_assume!(!x.is_nan() && !y.is_nan());

        // num_fields should always return the same value regardless of field values
        prop_assert_eq!(Point2D::num_fields(), 2);

        // Create instances with different values
        let _p1 = Point2D { x, y };
        let _p2 = Point2D { x: 0.0, y: 0.0 };

        // num_fields should still be 2
        prop_assert_eq!(Point2D::num_fields(), 2);
    }

    #[test]
    fn prop_array_length_matches_num_fields(x: f64, y: f64, z: f64) {
        prop_assume!(!x.is_nan() && !y.is_nan() && !z.is_nan());

        // The array type's length should match num_fields()
        let point = Point3D { x, y, z };
        let arr = point.to_arr();
        prop_assert_eq!(arr.len(), Point3D::num_fields());
    }
}

// Test String roundtrip with generated strings
proptest! {
    #[test]
    fn prop_generic_pair_string_roundtrip(s1 in "[a-z]{0,10}", s2 in "[a-z]{0,10}") {
        let original = GenericPair {
            first: s1.clone(),
            second: s2.clone()
        };
        let arr = original.clone().to_arr();
        let reconstructed = GenericPair::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }
}

// Test Vec<i32> roundtrip (non-Copy type)
proptest! {
    #[test]
    fn prop_generic_pair_vec_roundtrip(
        vec1 in prop::collection::vec(any::<i32>(), 0..5),
        vec2 in prop::collection::vec(any::<i32>(), 0..5)
    ) {
        let original = GenericPair {
            first: vec1.clone(),
            second: vec2.clone()
        };
        let arr = original.clone().to_arr();
        let reconstructed = GenericPair::from_arr(arr);

        prop_assert_eq!(original, reconstructed);
    }
}
