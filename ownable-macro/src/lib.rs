#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

//! This crate is not to be used on it's own, please see <https://docs.rs/ownable>.

mod attribute;
mod attribute_parser;
mod common;
mod r#enum;
mod generate;
mod mode;
mod r#struct;

use crate::attribute::DeriveAttribute;
use crate::mode::Mode;
use proc_macro_error::{abort, proc_macro_error};
use syn::{parse_macro_input, Data, DeriveInput};

/// Derive to_borrowed.
#[proc_macro_error]
#[proc_macro_derive(ToBorrowed, attributes(ownable))]
pub fn to_borrowed(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    derive(&input, Mode::ToBorrowed).into()
}

/// Derive to_owned.
#[proc_macro_error]
#[proc_macro_derive(ToOwned, attributes(ownable))]
pub fn to_owned(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    derive(&input, Mode::ToOwned).into()
}

/// Derive into_owned.
#[proc_macro_error]
#[proc_macro_derive(IntoOwned, attributes(ownable))]
pub fn into_owned(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    derive(&input, Mode::IntoOwned).into()
}

// a struct to just hold all global data together (rather than passing them always around)
pub(crate) struct Derive<'a> {
    pub(crate) input: &'a DeriveInput,
    pub(crate) attribute: &'a DeriveAttribute,
    pub(crate) mode: Mode,
}

fn derive(input: &DeriveInput, mode: Mode) -> proc_macro2::TokenStream {
    let attribute = &DeriveAttribute::new(&input.attrs);
    let derive = Derive {
        input,
        attribute,
        mode,
    };
    match &input.data {
        Data::Struct(data) => derive.derive_struct(data),
        Data::Enum(data) => derive.derive_enum(data),
        Data::Union(_data) => abort!(input, "union is not supported"),
    }
}
