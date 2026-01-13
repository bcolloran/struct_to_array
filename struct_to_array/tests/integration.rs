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
struct Tuple2(i32, i32);

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct Tuple3(String, String, String);

#[derive(StructToArray, Debug, PartialEq, Clone)]
struct GenericPair<T> {
    first: T,
    second: T,
}

#[test]
fn test_point2d_roundtrip() {
    let point = Point2D { x: 3.0, y: 4.0 };
    let arr = point.to_arr();
    assert_eq!(arr, [3.0, 4.0]);
    
    let reconstructed = Point2D::from_arr(arr);
    assert_eq!(reconstructed.x, 3.0);
    assert_eq!(reconstructed.y, 4.0);
}

#[test]
fn test_point3d_roundtrip() {
    let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let arr = point.to_arr();
    assert_eq!(arr, [1.0, 2.0, 3.0]);
    
    let reconstructed = Point3D::from_arr(arr);
    assert_eq!(reconstructed.x, 1.0);
    assert_eq!(reconstructed.y, 2.0);
    assert_eq!(reconstructed.z, 3.0);
}

#[test]
fn test_tuple2_roundtrip() {
    let tuple = Tuple2(10, 20);
    let arr = tuple.to_arr();
    assert_eq!(arr, [10, 20]);
    
    let reconstructed = Tuple2::from_arr(arr);
    assert_eq!(reconstructed.0, 10);
    assert_eq!(reconstructed.1, 20);
}

#[test]
fn test_tuple3_roundtrip() {
    let tuple = Tuple3("a".to_string(), "b".to_string(), "c".to_string());
    let arr = tuple.to_arr();
    assert_eq!(arr[0], "a");
    assert_eq!(arr[1], "b");
    assert_eq!(arr[2], "c");
    
    let reconstructed = Tuple3::from_arr(arr);
    assert_eq!(reconstructed.0, "a");
    assert_eq!(reconstructed.1, "b");
    assert_eq!(reconstructed.2, "c");
}

#[test]
fn test_generic_pair_roundtrip() {
    let pair = GenericPair { first: 100, second: 200 };
    let arr = pair.to_arr();
    assert_eq!(arr, [100, 200]);
    
    let reconstructed = GenericPair::from_arr(arr);
    assert_eq!(reconstructed.first, 100);
    assert_eq!(reconstructed.second, 200);
}

#[test]
fn test_generic_pair_string_roundtrip() {
    let pair = GenericPair {
        first: "hello".to_string(),
        second: "world".to_string(),
    };
    let arr = pair.to_arr();
    assert_eq!(arr[0], "hello");
    assert_eq!(arr[1], "world");
    
    let reconstructed = GenericPair::from_arr(arr);
    assert_eq!(reconstructed.first, "hello");
    assert_eq!(reconstructed.second, "world");
}

#[test]
fn test_num_fields() {
    assert_eq!(Point2D::num_fields(), 2);
    assert_eq!(Point3D::num_fields(), 3);
    assert_eq!(Tuple2::num_fields(), 2);
    assert_eq!(Tuple3::num_fields(), 3);
    assert_eq!(GenericPair::<i32>::num_fields(), 2);
}

#[test]
fn test_to_vec() {
    let point = Point2D { x: 5.0, y: 6.0 };
    let vec = point.to_vec();
    assert_eq!(vec, vec![5.0, 6.0]);
}

#[test]
fn test_from_vec() {
    let vec = vec![7.0, 8.0];
    let point = Point2D::from_vec(&vec);
    assert_eq!(point.x, 7.0);
    assert_eq!(point.y, 8.0);
}

#[test]
#[should_panic(expected = "Input vec length 3 does not match expected 2")]
fn test_from_vec_wrong_length_panics() {
    let vec = vec![1.0, 2.0, 3.0];
    let _point = Point2D::from_vec(&vec);
}

#[test]
fn test_move_semantics_no_copy_required() {
    // This test verifies that the macro works with non-Copy types
    #[derive(StructToArray, Debug, PartialEq)]
    struct NonCopyPair {
        first: Vec<i32>,
        second: Vec<i32>,
    }
    
    let pair = NonCopyPair {
        first: vec![1, 2, 3],
        second: vec![4, 5, 6],
    };
    
    let arr = pair.to_arr();
    assert_eq!(arr[0], vec![1, 2, 3]);
    assert_eq!(arr[1], vec![4, 5, 6]);
    
    let reconstructed = NonCopyPair::from_arr(arr);
    assert_eq!(reconstructed.first, vec![1, 2, 3]);
    assert_eq!(reconstructed.second, vec![4, 5, 6]);
}

#[test]
fn test_large_struct() {
    #[derive(StructToArray, Debug, PartialEq)]
    struct Large {
        a: i32, b: i32, c: i32, d: i32, e: i32,
        f: i32, g: i32, h: i32, i: i32, j: i32,
    }
    
    let large = Large {
        a: 1, b: 2, c: 3, d: 4, e: 5,
        f: 6, g: 7, h: 8, i: 9, j: 10,
    };
    
    let arr = large.to_arr();
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    let reconstructed = Large::from_arr(arr);
    assert_eq!(reconstructed.a, 1);
    assert_eq!(reconstructed.j, 10);
}
