use crate::Derive;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields};

impl Derive<'_> {
    pub(crate) fn derive_struct(&self, data: &DataStruct) -> TokenStream {
        self.generate(&match &data.fields {
            Fields::Named(data) => self.derive_named(None, data),
            Fields::Unnamed(data) => self.derive_unnamed(None, data),
            Fields::Unit => self.derive_struct_unit(),
        })
    }

    fn derive_struct_unit(&self) -> TokenStream {
        let name = &self.input.ident;
        quote! {#name}
    }
}
