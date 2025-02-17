#![allow(clippy::useless_conversion)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta};

#[proc_macro_derive(Maydon, attributes(field_name))]
pub fn field_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Default enum name
    let mut enum_name = syn::Ident::new("Field", proc_macro2::Span::call_site());

    // Parse attributes to extract custom enum name
    for attr in input.attrs.iter() {
        if attr.path().is_ident("field_name") {
            if let Ok(Meta::NameValue(name_value)) = attr.meta.clone().try_into() {
                // Extract value from Meta::NameValue
                if let Expr::Lit(expr_lit) = name_value.value {
                    if let Lit::Str(lit_str) = expr_lit.lit {
                        enum_name =
                            syn::Ident::new(&lit_str.value(), proc_macro2::Span::call_site());
                    }
                }
            }
        }
    }

    // Extract fields from the struct
    let fields = if let syn::Data::Struct(data_struct) = &input.data {
        &data_struct.fields
    } else {
        panic!("Maydon can only be derived for structs");
    };

    let field_variants = fields.iter().map(|field| {
        let field_name = &field.ident;
        let variant_name = syn::Ident::new(
            &field_name
                .as_ref()
                .unwrap()
                .to_string()
                .to_case(Case::Pascal),
            proc_macro2::Span::call_site(),
        );
        quote! { #variant_name }
    });

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_name {
            #(#field_variants,)*
            Unknown,
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn test_macro() {
        let t = TestCases::new();
        t.pass("tests/basic.rs");
        t.pass("tests/custom.rs");
        t.compile_fail("tests/fail.rs");
    }
}
