use pretty_assertions::assert_eq;
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
        impl ::struct_to_array::StructToArray<f32> for Point2D {
            type Arr = [f32; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.x, self.y]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [x, y] = arr;
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
        impl ::struct_to_array::StructToArray<f64> for Point3D {
            type Arr = [f64; 3];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.x, self.y, self.z]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [x, y, z] = arr;
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
        impl ::struct_to_array::StructToArray<i32> for SingleValue {
            type Arr = [i32; 1];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.value]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [value] = arr;
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
        impl ::struct_to_array::StructToArray<f32> for ManyFloats {
            type Arr = [f32; 5];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.a, self.b, self.c, self.d, self.e]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [a, b, c, d, e] = arr;
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
        impl ::struct_to_array::StructToArray<Vec<String> > for ComplexPair {
            type Arr = [Vec<String>; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.first, self.second]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [first, second] = arr;
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
        impl ::struct_to_array::StructToArray<f32> for PublicPoint {
            type Arr = [f32; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.x, self.y]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [x, y] = arr;
                Self { x, y }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}
