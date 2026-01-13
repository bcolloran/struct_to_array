use quote::quote;
use syn::{DeriveInput, parse_quote};

/// Helper function that expects the macro to fail
fn expect_error(input: DeriveInput, expected_msg_fragment: &str) {
    let result = crate::expand_struct_to_array(&input);
    assert!(
        result.is_err(),
        "Expected error but got success for input:\n{}",
        quote!(#input)
    );
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains(expected_msg_fragment),
        "Error message '{}' does not contain expected fragment '{}'",
        err_msg,
        expected_msg_fragment
    );
}

#[test]
fn test_unit_struct_fails() {
    let input: DeriveInput = parse_quote! {
        struct UnitStruct;
    };

    expect_error(input, "cannot be derived for unit structs");
}

#[test]
fn test_enum_fails() {
    let input: DeriveInput = parse_quote! {
        enum MyEnum {
            A,
            B,
        }
    };

    expect_error(input, "can only be derived for structs");
}

#[test]
fn test_empty_named_struct_fails() {
    let input: DeriveInput = parse_quote! {
        struct Empty {}
    };

    expect_error(input, "requires at least one field");
}

#[test]
fn test_empty_tuple_struct_fails() {
    let input: DeriveInput = parse_quote! {
        struct Empty();
    };

    expect_error(input, "requires at least one field");
}

#[test]
fn test_mixed_types_named_fails() {
    let input: DeriveInput = parse_quote! {
        struct MixedTypes {
            x: f32,
            y: i32,
        }
    };

    expect_error(input, "identical type tokens");
}

#[test]
fn test_mixed_types_tuple_fails() {
    let input: DeriveInput = parse_quote! {
        struct MixedTypes(String, i32);
    };

    expect_error(input, "identical type tokens");
}

#[test]
fn test_mixed_types_three_fields_first_two_match() {
    let input: DeriveInput = parse_quote! {
        struct MixedTypes {
            a: f32,
            b: f32,
            c: i32,
        }
    };

    expect_error(input, "identical type tokens");
}

#[test]
fn test_mixed_types_three_fields_last_two_match() {
    let input: DeriveInput = parse_quote! {
        struct MixedTypes {
            a: i32,
            b: f32,
            c: f32,
        }
    };

    expect_error(input, "identical type tokens");
}

#[test]
fn test_similar_but_different_types_fail() {
    // Vec<i32> vs Vec<u32> should fail
    let input: DeriveInput = parse_quote! {
        struct SimilarTypes {
            a: Vec<i32>,
            b: Vec<u32>,
        }
    };

    expect_error(input, "identical type tokens");
}

#[test]
fn test_different_references_fail() {
    // &i32 vs &mut i32 should fail
    let input: DeriveInput = parse_quote! {
        struct RefTypes {
            a: &'static i32,
            b: &'static mut i32,
        }
    };

    expect_error(input, "identical type tokens");
}
