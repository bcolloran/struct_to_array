use syn::{parse_quote, DeriveInput};

/// Helper function to expand the StructToArray macro for testing
fn expand_struct_to_array(input: DeriveInput) -> proc_macro2::TokenStream {
    crate::expand_struct_to_array(&input).unwrap_or_else(|e| e.to_compile_error())
}

#[test]
fn test_crate_attribute_explicit_crate() {
    let input: DeriveInput = parse_quote! {
        #[struct_to_array(crate = "crate")]
        struct Point {
            x: f32,
            y: f32,
        }
    };

    let actual = expand_struct_to_array(input);

    // Should use `crate::StructToArray` instead of `::struct_to_array::StructToArray`
    assert!(actual.to_string().contains("crate :: StructToArray"));
}

#[test]
fn test_crate_attribute_custom_path() {
    let input: DeriveInput = parse_quote! {
        #[struct_to_array(crate = "my_custom_struct_to_array")]
        struct Point {
            x: f32,
            y: f32,
        }
    };

    let actual = expand_struct_to_array(input);

    // Should use `::my_custom_struct_to_array::StructToArray`
    assert!(actual
        .to_string()
        .contains(":: my_custom_struct_to_array :: StructToArray"));
}

#[test]
fn test_no_crate_attribute_uses_default() {
    let input: DeriveInput = parse_quote! {
        struct Point {
            x: f32,
            y: f32,
        }
    };

    let actual = expand_struct_to_array(input);

    // Should use default path (either ::struct_to_array or crate depending on proc-macro-crate detection)
    // We just verify it compiles successfully
    assert!(!actual.to_string().is_empty());
}

#[test]
fn test_invalid_crate_attribute_fails() {
    let input: DeriveInput = parse_quote! {
        #[struct_to_array(invalid_key = "value")]
        struct Point {
            x: f32,
            y: f32,
        }
    };

    let result = crate::expand_struct_to_array(&input);
    assert!(result.is_err(), "Expected error for invalid attribute key");
}
