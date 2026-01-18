use pretty_assertions::assert_eq;
use quote::quote;
use syn::{DeriveInput, parse_quote};

/// Helper function to expand the StructToArray macro for testing
fn expand_struct_to_array(input: DeriveInput) -> proc_macro2::TokenStream {
    crate::expand_struct_to_array(&input).unwrap_or_else(|e| e.to_compile_error())
}

#[test]
fn test_generic_type_single_param() {
    let input: DeriveInput = parse_quote! {
        struct GenericPair<T> {
            first: T,
            second: T,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<T> ::struct_to_array::StructToArray<T> for GenericPair<T> {
            type Arr = [T; 2];

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
fn test_generic_type_with_bounds() {
    let input: DeriveInput = parse_quote! {
        struct BoundedPair<T: Clone> {
            first: T,
            second: T,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<T: Clone> ::struct_to_array::StructToArray<T> for BoundedPair<T> {
            type Arr = [T; 2];

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
fn test_generic_type_where_clause() {
    let input: DeriveInput = parse_quote! {
        struct WherePair<T>
        where
            T: Clone + Send,
        {
            first: T,
            second: T,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<T> ::struct_to_array::StructToArray<T> for WherePair<T>
        where
            T: Clone + Send,
        {
            type Arr = [T; 2];

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
fn test_generic_tuple_struct() {
    let input: DeriveInput = parse_quote! {
        struct GenericTriple<T>(T, T, T);
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<T> ::struct_to_array::StructToArray<T> for GenericTriple<T> {
            type Arr = [T; 3];

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
fn test_lifetime_parameters() {
    let input: DeriveInput = parse_quote! {
        struct RefPair<'a> {
            first: &'a str,
            second: &'a str,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<'a> ::struct_to_array::StructToArray<&'a str> for RefPair<'a> {
            type Arr = [&'a str; 2];

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
fn test_multiple_lifetimes() {
    let input: DeriveInput = parse_quote! {
        struct MultiRefPair<'a, 'b> {
            first: &'a str,
            second: &'a str,
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<'a, 'b> ::struct_to_array::StructToArray<&'a str> for MultiRefPair<'a, 'b> {
            type Arr = [&'a str; 2];

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
fn test_const_generic_in_field_type() {
    let input: DeriveInput = parse_quote! {
        struct ArrayPair<const N: usize> {
            first: [u8; N],
            second: [u8; N],
        }
    };

    let actual = expand_struct_to_array(input);
    let expected = quote! {
        impl<const N: usize> ::struct_to_array::StructToArray<[u8; N]> for ArrayPair<N> {
            type Arr = [[u8; N]; 2];

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
