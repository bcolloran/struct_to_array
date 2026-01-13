use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Error, Fields, Ident, LitStr, parse_macro_input, spanned::Spanned};

#[cfg(test)]
mod tests;

/// Derive `struct_to_array::StructToArray<Item, N>` for a struct whose fields are all the same
/// type (token-identical).
///
/// Supported:
/// - named-field structs: `struct Foo { a: T, b: T }`
/// - tuple structs: `struct Foo(T, T)`
///
/// Not supported:
/// - unit structs: `struct Foo;`
/// - structs with zero fields (no way to infer `Item`)
///
/// Optional attribute:
/// - `#[struct_to_array(crate = "path_ident")]`
///   Use if you renamed the `struct_to_array` dependency in Cargo.toml.
///   Example: `struct_to_array = { package = "struct_to_array", version = "...", package = "...", default-features = false }`
///   (you usually won’t need this; `proc-macro-crate` handles common renames)
#[proc_macro_derive(StructToArray, attributes(struct_to_array))]
pub fn derive_struct_to_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_struct_to_array(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_struct_to_array(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let data_struct = match &input.data {
        Data::Struct(s) => s,
        _ => {
            return Err(Error::new_spanned(
                name,
                "StructToArray can only be derived for structs",
            ));
        }
    };

    let trait_crate = resolve_trait_crate_path(&input.attrs)?;
    let trait_path = quote!(#trait_crate::StructToArray);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &data_struct.fields {
        Fields::Named(fields_named) => {
            let fields: Vec<_> = fields_named.named.iter().collect();
            if fields.is_empty() {
                return Err(Error::new(
                    name.span(),
                    "StructToArray requires at least one field",
                ));
            }

            let item_ty = &fields[0].ty;
            let item_ty_str = item_ty.to_token_stream().to_string();

            for f in fields.iter().skip(1) {
                let f_ty_str = f.ty.to_token_stream().to_string();
                if f_ty_str != item_ty_str {
                    return Err(Error::new(
                        f.ty.span(),
                        format!(
                            "StructToArray requires all fields to have identical type tokens; expected `{}`, found `{}`",
                            item_ty_str, f_ty_str
                        ),
                    ));
                }
            }

            let field_idents: Vec<Ident> = fields
                .iter()
                .map(|f| f.ident.clone().expect("named field must have ident"))
                .collect();

            let n = field_idents.len();
            let n_lit = syn::LitInt::new(&n.to_string(), Span::call_site());

            let to_elems = field_idents.iter().map(|id| quote!(self.#id));

            // Move out of the array without requiring `Item: Copy`:
            // let [x, y, z] = a;
            // Self { x, y, z }
            let expanded = quote! {
                impl #impl_generics #trait_path<#item_ty, #n_lit> for #name #ty_generics #where_clause {
                    const N_ATOMS: usize = #n_lit;

                    #[inline]
                    fn to_arr(self) -> [#item_ty; #n_lit] {
                        [#(#to_elems),*]
                    }

                    #[inline]
                    fn from_arr(a: [#item_ty; #n_lit]) -> Self {
                        let [#(#field_idents),*] = a;
                        Self { #(#field_idents),* }
                    }
                }
            };

            Ok(expanded)
        }

        Fields::Unnamed(fields_unnamed) => {
            let fields: Vec<_> = fields_unnamed.unnamed.iter().collect();
            if fields.is_empty() {
                return Err(Error::new(
                    name.span(),
                    "StructToArray requires at least one field",
                ));
            }

            let item_ty = &fields[0].ty;
            let item_ty_str = item_ty.to_token_stream().to_string();

            for f in fields.iter().skip(1) {
                let f_ty_str = f.ty.to_token_stream().to_string();
                if f_ty_str != item_ty_str {
                    return Err(Error::new(
                        f.ty.span(),
                        format!(
                            "StructToArray requires all fields to have identical type tokens; expected `{}`, found `{}`",
                            item_ty_str, f_ty_str
                        ),
                    ));
                }
            }

            let n = fields.len();
            let n_lit = syn::LitInt::new(&n.to_string(), Span::call_site());

            let idxs: Vec<syn::Index> = (0..n).map(syn::Index::from).collect();
            let to_elems = idxs.iter().map(|i| quote!(self.#i));

            // Bindings for destructuring the array: let [v0, v1, ...] = a;
            let binds: Vec<Ident> = (0..n)
                .map(|i| Ident::new(&format!("__v{}", i), Span::call_site()))
                .collect();

            let expanded = quote! {
                impl #impl_generics #trait_path<#item_ty, #n_lit> for #name #ty_generics #where_clause {

                    const N_ATOMS: usize = #n_lit;

                    #[inline]
                    fn to_arr(self) -> [#item_ty; #n_lit] {
                        [#(#to_elems),*]
                    }

                    #[inline]
                    fn from_arr(a: [#item_ty; #n_lit]) -> Self {
                        let [#(#binds),*] = a;
                        Self(#(#binds),*)
                    }
                }
            };

            Ok(expanded)
        }

        Fields::Unit => Err(Error::new_spanned(
            name,
            "StructToArray cannot be derived for unit structs",
        )),
    }
}

fn resolve_trait_crate_path(attrs: &[syn::Attribute]) -> syn::Result<proc_macro2::TokenStream> {
    // 1) Optional explicit override: #[struct_to_array(crate = "foo")]
    for attr in attrs {
        if !attr.path().is_ident("struct_to_array") {
            continue;
        }

        let mut override_name: Option<LitStr> = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let lit: LitStr = meta.value()?.parse()?;
                override_name = Some(lit);
                Ok(())
            } else {
                Err(meta.error("supported: #[struct_to_array(crate = \"...\")]"))
            }
        })?;

        if let Some(lit) = override_name {
            let s = lit.value();
            if s == "crate" {
                return Ok(quote!(crate));
            }
            let ident = Ident::new(&s, lit.span());
            return Ok(quote!(::#ident));
        }
    }

    // 2) Otherwise resolve via proc-macro-crate (handles dependency renames)
    match proc_macro_crate::crate_name("struct_to_array") {
        Ok(proc_macro_crate::FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
        Err(_) => Ok(quote!(::struct_to_array)), // fallback
    }
}
