use quote::quote;
use syn::{DeriveInput, parse_quote};

/// Helper function to expand the StructToArray macro for testing
fn expand_struct_to_array(input: DeriveInput) -> proc_macro2::TokenStream {
    crate::expand_struct_to_array(&input).unwrap_or_else(|e| e.to_compile_error())
}

#[test]
fn test_named_fields_two_fields() {
    let input: DeriveInput = parse_quote! {
        struct Point2D {
            x: f32,
            y: f32,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f32, 2> for Point2D {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [f32; 2] {
                [self.x, self.y]
            }

            #[inline]
            fn from_arr(a: [f32; 2]) -> Self {
                let [x, y] = a;
                Self { x, y }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_named_fields_three_fields() {
    let input: DeriveInput = parse_quote! {
        struct Point3D {
            x: f64,
            y: f64,
            z: f64,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f64, 3> for Point3D {
            const N_ATOMS: usize = 3;

            #[inline]
            fn to_arr(self) -> [f64; 3] {
                [self.x, self.y, self.z]
            }

            #[inline]
            fn from_arr(a: [f64; 3]) -> Self {
                let [x, y, z] = a;
                Self { x, y, z }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_named_fields_single_field() {
    let input: DeriveInput = parse_quote! {
        struct SingleValue {
            value: i32,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<i32, 1> for SingleValue {
            const N_ATOMS: usize = 1;

            #[inline]
            fn to_arr(self) -> [i32; 1] {
                [self.value]
            }

            #[inline]
            fn from_arr(a: [i32; 1]) -> Self {
                let [value] = a;
                Self { value }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_named_fields_many_fields() {
    let input: DeriveInput = parse_quote! {
        struct ManyFloats {
            a: f32,
            b: f32,
            c: f32,
            d: f32,
            e: f32,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f32, 5> for ManyFloats {
            const N_ATOMS: usize = 5;

            #[inline]
            fn to_arr(self) -> [f32; 5] {
                [self.a, self.b, self.c, self.d, self.e]
            }

            #[inline]
            fn from_arr(a: [f32; 5]) -> Self {
                let [a, b, c, d, e] = a;
                Self { a, b, c, d, e }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_named_fields_complex_type() {
    let input: DeriveInput = parse_quote! {
        struct ComplexPair {
            first: Vec<String>,
            second: Vec<String>,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<Vec<String>, 2> for ComplexPair {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [Vec<String>; 2] {
                [self.first, self.second]
            }

            #[inline]
            fn from_arr(a: [Vec<String>; 2]) -> Self {
                let [first, second] = a;
                Self { first, second }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_named_fields_with_pub() {
    let input: DeriveInput = parse_quote! {
        pub struct PublicPoint {
            pub x: f32,
            pub y: f32,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f32, 2> for PublicPoint {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [f32; 2] {
                [self.x, self.y]
            }

            #[inline]
            fn from_arr(a: [f32; 2]) -> Self {
                let [x, y] = a;
                Self { x, y }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}
