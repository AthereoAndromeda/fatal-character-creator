use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(Summable)]
pub fn derive_sum(tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let ident = &item.ident;

    match &item.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut v = Vec::new();
            let field_len = fields.len();

            for field in fields {
                // Gets only i32 fields
                if matches!(&field.ty, syn::Type::Path(path) if path.qself.is_none() && path.path.is_ident("i32"))
                {
                    let id = field.ident.as_ref().unwrap();
                    v.push(id);
                }
            }

            quote! {
                #[automatically_derived]
                impl ::std::ops::Add for #ident {
                    type Output = Self;
                    fn add(self, rhs: Self) -> Self::Output {
                        Self {
                            #(#v: self.#v + rhs.#v),*
                        }
                    }
                }

                #[automatically_derived]
                impl #ident {
                    pub fn sum(&self) -> i32 {
                        0 #(+ self.#v )*
                    }

                    pub fn avg(&self) -> i32 {
                        self.sum() / #field_len as i32
                    }
                }
            }
            .into()
        }

        _ => unimplemented!(),
    }
}

#[proc_macro_derive(GenderModifiers)]
pub fn apply_gender_modifiers(tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let ident = item.ident;

    match &item.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut v = Vec::new();

            for field in fields {
                let id = field.ident.as_ref().unwrap();
                v.push(id);
            }

            let ident_v = v
                .iter()
                .map(|x| format_ident!("var_{}", x))
                .collect::<Vec<_>>();

            quote! {
                impl #ident {
                    pub fn apply_gender_modifiers(&self, modifiers: Self) -> Self {
                        #(
                            let #ident_v = (self.#v * ((100 + modifiers.#v))).div_euclid(100);
                        )*

                        Self {
                            #(#v: #ident_v),*
                        }
                    }
                }
            }
            .into()
        }

        _ => unimplemented!(),
    }
}
