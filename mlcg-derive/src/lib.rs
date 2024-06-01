#[proc_macro_derive(Eval)]
pub fn derive_eval_for_self(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote::quote! {
        impl #impl_generics Eval<#name #ty_generics> for #name #ty_generics #where_clause {
            #[inline]
            fn eval(self) -> #name #ty_generics {
                self
            }
        }
    };
    expanded.into()
}
