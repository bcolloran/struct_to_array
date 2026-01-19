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

#[test]
fn test_regression_case_1_point2d_zero() {
    // Regression case from proptest: x = 0.0
    // This appears to be testing with x = 0.0 in a wrong-length scenario
    // Based on the regression file line:
    // cc 63c31064a7479b4cd56d0b30ab0111f901c6ce320c569575a1bd92f47e5eeb2c # shrinks to x = 0.0

    // The actual issue is that the should_panic test was incorrectly written
    // and was not actually testing what it should. Let's verify the panic happens correctly:
    let result = std::panic::catch_unwind(|| {
        let vec = vec![0.0_f32];
        let _point = Point2D::from_vec(&vec);
    });
    assert!(result.is_err(), "from_vec should panic with wrong length");
}

#[test]
fn test_regression_case_2_point2d_wrong_length_zeros() {
    // Regression case from proptest: x = 0.0, y = 0.0, z = 0.0
    // Based on the regression file line:
    // cc 98482495f07d1a92f7ed1bfcda73e819c1d765482705514d0c3f6899449e6fc2 # shrinks to x = 0.0, y = 0.0, z = 0.0

    // The actual issue is that the should_panic test was incorrectly written
    // Let's verify the panic happens correctly:
    let result = std::panic::catch_unwind(|| {
        let vec = vec![0.0_f32, 0.0_f32, 0.0_f32];
        let _point = Point2D::from_vec(&vec);
    });
    assert!(
        result.is_err(),
        "from_vec should panic with wrong length (3 instead of 2)"
    );
}

#[test]
fn test_point2d_roundtrip_with_zeros() {
    // Also verify that legitimate zero values work correctly
    let original = Point2D { x: 0.0, y: 0.0 };
    let vec = original.clone().to_vec();
    let reconstructed = Point2D::from_vec(&vec);
    assert_eq!(original, reconstructed);
}

#[test]
fn test_point3d_roundtrip_with_zeros() {
    // Also verify that legitimate zero values work correctly
    let original = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vec = original.clone().to_vec();
    let reconstructed = Point3D::from_vec(&vec);
    assert_eq!(original, reconstructed);
}
