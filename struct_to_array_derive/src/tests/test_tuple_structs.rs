use pretty_assertions::assert_eq;
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
        impl ::struct_to_array::StructToArray<f32> for Pair {
            type Arr = [f32; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0, __v1] = arr;
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
        impl ::struct_to_array::StructToArray<i32> for Triple {
            type Arr = [i32; 3];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0, self.1, self.2]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0, __v1, __v2] = arr;
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
        impl ::struct_to_array::StructToArray<String> for Wrapper {
            type Arr = [String; 1];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0] = arr;
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
        impl ::struct_to_array::StructToArray<u64> for ManyInts {
            type Arr = [u64; 6];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0, self.1, self.2, self.3, self.4, self.5]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0, __v1, __v2, __v3, __v4, __v5] = arr;
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
        impl ::struct_to_array::StructToArray<Vec<i32> > for VecPair {
            type Arr = [Vec<i32>; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0, __v1] = arr;
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
        impl ::struct_to_array::StructToArray<f64> for PublicPair {
            type Arr = [f64; 2];

            #[inline]
            fn to_arr(self) -> Self::Arr {
                [self.0, self.1]
            }

            #[inline]
            fn from_arr(arr: Self::Arr) -> Self {
                let [__v0, __v1] = arr;
                Self(__v0, __v1)
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}
