use struct_to_array::{StructToArray, StructToVec};

#[derive(StructToArray, Debug, PartialEq)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(StructToArray, Debug, PartialEq)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(StructToArray, Debug, PartialEq)]
struct Tuple2(i32, i32);

#[derive(StructToArray, Debug, PartialEq)]
struct Tuple3(u64, u64, u64);

#[test]
fn test_point2d_roundtrip() {
    let point = Point2D { x: 1.0, y: 2.0 };
    let arr = point.to_arr();
    assert_eq!(arr, [1.0, 2.0]);
    let point2 = Point2D::from_arr(arr);
    assert_eq!(point2.x, 1.0);
    assert_eq!(point2.y, 2.0);
}

#[test]
fn test_point3d_roundtrip() {
    let point = Point3D {
        x: 1.5,
        y: 2.5,
        z: 3.5,
    };
    let arr = point.to_arr();
    assert_eq!(arr, [1.5, 2.5, 3.5]);
    let point2 = Point3D::from_arr(arr);
    assert_eq!(
        point2,
        Point3D {
            x: 1.5,
            y: 2.5,
            z: 3.5
        }
    );
}

#[test]
fn test_tuple2_roundtrip() {
    let tup = Tuple2(10, 20);
    let arr = tup.to_arr();
    assert_eq!(arr, [10, 20]);
    let tup2 = Tuple2::from_arr(arr);
    assert_eq!(tup2, Tuple2(10, 20));
}

#[test]
fn test_tuple3_roundtrip() {
    let tup = Tuple3(100, 200, 300);
    let arr = tup.to_arr();
    assert_eq!(arr, [100, 200, 300]);
    let tup2 = Tuple3::from_arr(arr);
    assert_eq!(tup2, Tuple3(100, 200, 300));
}

#[test]
fn test_num_fields() {
    assert_eq!(Point2D::num_fields(), 2);
    assert_eq!(Point3D::num_fields(), 3);
    assert_eq!(Tuple2::num_fields(), 2);
    assert_eq!(Tuple3::num_fields(), 3);
}

#[test]
fn test_to_vec() {
    let point = Point2D { x: 5.0, y: 10.0 };
    let vec = point.to_vec();
    assert_eq!(vec, vec![5.0, 10.0]);
}

#[test]
fn test_from_vec() {
    let vec = vec![3.0, 4.0];
    let point = Point2D::from_vec(&vec);
    assert_eq!(point, Point2D { x: 3.0, y: 4.0 });
}

#[test]
#[should_panic(expected = "Input vec length 3 does not match expected 2")]
fn test_from_vec_wrong_length_panics() {
    let vec = vec![1.0, 2.0, 3.0];
    let _point = Point2D::from_vec(&vec);
}

#[test]
fn test_move_semantics_no_copy_required() {
    // String doesn't implement Copy, so this tests move semantics
    #[derive(StructToArray, Debug)]
    struct Strings {
        a: String,
        b: String,
    }

    let strings = Strings {
        a: "hello".to_string(),
        b: "world".to_string(),
    };

    let arr = strings.to_arr();
    assert_eq!(arr[0], "hello");
    assert_eq!(arr[1], "world");

    let strings2 = Strings::from_arr(arr);
    assert_eq!(strings2.a, "hello");
    assert_eq!(strings2.b, "world");
}

#[test]
fn test_vec_with_clone() {
    #[derive(StructToArray, Debug, PartialEq)]
    struct Words {
        a: String,
        b: String,
    }

    let words = Words {
        a: "foo".to_string(),
        b: "bar".to_string(),
    };

    let vec = words.to_vec();
    assert_eq!(vec, vec!["foo".to_string(), "bar".to_string()]);

    let words2 = Words::from_vec(&vec);
    assert_eq!(
        words2,
        Words {
            a: "foo".to_string(),
            b: "bar".to_string(),
        }
    );
}

#[test]
fn test_large_struct() {
    #[derive(StructToArray, Debug, PartialEq)]
    struct Large {
        f1: i32,
        f2: i32,
        f3: i32,
        f4: i32,
        f5: i32,
        f6: i32,
        f7: i32,
        f8: i32,
    }

    let large = Large {
        f1: 1,
        f2: 2,
        f3: 3,
        f4: 4,
        f5: 5,
        f6: 6,
        f7: 7,
        f8: 8,
    };

    assert_eq!(Large::num_fields(), 8);

    let arr = large.to_arr();
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);

    let large2 = Large::from_arr(arr);
    assert_eq!(
        large2,
        Large {
            f1: 1,
            f2: 2,
            f3: 3,
            f4: 4,
            f5: 5,
            f6: 6,
            f7: 7,
            f8: 8,
        }
    );
}

#[test]
fn test_generic_struct() {
    #[derive(StructToArray, Debug, PartialEq)]
    struct Pair<T> {
        first: T,
        second: T,
    }

    let pair = Pair {
        first: 42,
        second: 84,
    };
    let arr = pair.to_arr();
    assert_eq!(arr, [42, 84]);

    let pair2 = Pair::from_arr(arr);
    assert_eq!(
        pair2,
        Pair {
            first: 42,
            second: 84
        }
    );
}

#[test]
fn test_single_field_struct() {
    #[derive(StructToArray, Debug, PartialEq)]
    struct Single {
        value: f32,
    }

    let single = Single { value: 2.5 };
    let arr = single.to_arr();
    assert_eq!(arr, [2.5]);

    let single2 = Single::from_arr(arr);
    assert_eq!(single2, Single { value: 2.5 });
}
