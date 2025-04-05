use crate::attribute::DeriveAttribute;
use crate::mode::Mode;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort;
use syn::{Data, DeriveInput, Generics};

// a struct to just hold all global data together (rather than passing them always around)
pub(crate) struct Derive<'a> {
    pub(crate) input: &'a DeriveInput,
    pub(crate) ident: &'a Ident,
    pub(crate) generics: &'a Generics,
    pub(crate) attribute: &'a DeriveAttribute,
    pub(crate) mode: Mode,
}

pub(crate) fn derive(input: &DeriveInput, mode: Mode) -> TokenStream {
    let attribute = &DeriveAttribute::new(input);
    let derive = Derive {
        input,
        ident: &input.ident,
        generics: &input.generics,
        attribute,
        mode,
    };
    match &input.data {
        Data::Struct(data) => derive.derive_struct(data),
        Data::Enum(data) => derive.derive_enum(data),
        Data::Union(_data) => abort!(input, "union is not supported"),
    }
}
