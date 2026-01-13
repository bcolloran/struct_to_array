use quote::quote;
use syn::{DeriveInput, parse_quote};

/// Helper function to expand the StructToArray macro for testing
fn expand_struct_to_array(input: DeriveInput) -> proc_macro2::TokenStream {
    crate::expand_struct_to_array(&input).unwrap_or_else(|e| e.to_compile_error())
}

#[test]
fn test_tuple_struct_two_fields() {
    let input: DeriveInput = parse_quote! {
        struct Pair(f32, f32);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f32, 2> for Pair {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [f32; 2] {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(a: [f32; 2]) -> Self {
                let [__v0, __v1] = a;
                Self(__v0, __v1)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_three_fields() {
    let input: DeriveInput = parse_quote! {
        struct Triple(i32, i32, i32);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<i32, 3> for Triple {
            const N_ATOMS: usize = 3;

            #[inline]
            fn to_arr(self) -> [i32; 3] {
                [self.0, self.1, self.2]
            }

            #[inline]
            fn from_arr(a: [i32; 3]) -> Self {
                let [__v0, __v1, __v2] = a;
                Self(__v0, __v1, __v2)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_single_field() {
    let input: DeriveInput = parse_quote! {
        struct Wrapper(String);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<String, 1> for Wrapper {
            const N_ATOMS: usize = 1;

            #[inline]
            fn to_arr(self) -> [String; 1] {
                [self.0]
            }

            #[inline]
            fn from_arr(a: [String; 1]) -> Self {
                let [__v0] = a;
                Self(__v0)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_many_fields() {
    let input: DeriveInput = parse_quote! {
        struct ManyInts(u64, u64, u64, u64, u64, u64);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<u64, 6> for ManyInts {
            const N_ATOMS: usize = 6;

            #[inline]
            fn to_arr(self) -> [u64; 6] {
                [self.0, self.1, self.2, self.3, self.4, self.5]
            }

            #[inline]
            fn from_arr(a: [u64; 6]) -> Self {
                let [__v0, __v1, __v2, __v3, __v4, __v5] = a;
                Self(__v0, __v1, __v2, __v3, __v4, __v5)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_complex_type() {
    let input: DeriveInput = parse_quote! {
        struct VecPair(Vec<i32>, Vec<i32>);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<Vec<i32>, 2> for VecPair {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [Vec<i32>; 2] {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(a: [Vec<i32>; 2]) -> Self {
                let [__v0, __v1] = a;
                Self(__v0, __v1)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_with_pub() {
    let input: DeriveInput = parse_quote! {
        pub struct PublicPair(pub f64, pub f64);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl ::struct_to_array::StructToArray<f64, 2> for PublicPair {
            const N_ATOMS: usize = 2;

            #[inline]
            fn to_arr(self) -> [f64; 2] {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(a: [f64; 2]) -> Self {
                let [__v0, __v1] = a;
                Self(__v0, __v1)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}
