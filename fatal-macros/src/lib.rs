use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Summable)]
pub fn derive_sum(tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let ident = &item.ident;

    match &item.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut v = Vec::new();

            for field in fields {
                if matches!(&field.ty, syn::Type::Path(path) if path.qself.is_none() && path.path.is_ident("i32"))
                {
                    let id = field.ident.as_ref().unwrap();
                    v.push(quote! {
                        self.#id
                    });
                }
            }

            quote! {
                #[automatically_derived]
                impl #ident {
                    fn sum(&self) -> i32 {
                        0 #(+ #v )*
                    }
                }
            }
            .into()
        }

        _ => unimplemented!(),
    }
}
