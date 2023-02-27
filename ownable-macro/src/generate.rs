use crate::mode::Mode;
use crate::Derive;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::{GenericParam, Generics, Lifetime, LifetimeParam};

impl Derive<'_> {
    pub(crate) fn generate(&self, inner: &TokenStream) -> TokenStream {
        match self.mode {
            Mode::ToBorrowed => self.generate_mode_to_borrowed(inner),
            Mode::ToOwned | Mode::IntoOwned => self.generate_mode_in_to_owned(inner),
        }
    }

    fn generate_mode_to_borrowed(&self, inner: &TokenStream) -> TokenStream {
        if self.input.generics.lifetimes().count() == 0 {
            abort!(
                self.input,
                "ToBorrowed can be only derived for a struct with a lifetime"
            );
        }
        let name = &self.input.ident;
        let def_generics = self.generate_generics(None, false);
        let generics_dedup = self.generate_generics(Some("'a"), true);
        let generics = self.generate_generics(Some("'a"), false);
        let use_generics = self.generate_generics(Some("'_"), false);
        let trait_name = Mode::ToBorrowed.name();
        let doc = Mode::ToBorrowed.doc();
        quote! {
            impl #generics_dedup #trait_name <'a> for #name #generics
            {
                fn to_borrowed(&'a self) -> Self {
                    #inner
                }
            }

            impl #def_generics #name #use_generics {
                #[doc=#doc]
                #[inline(always)]
                pub fn to_borrowed(& self) -> #name #def_generics {
                    #trait_name::to_borrowed(self)
                }
            }
        }
    }

    fn generate_mode_in_to_owned(&self, inner: &TokenStream) -> TokenStream {
        let name = &self.input.ident;
        let def_generics = self.generate_generics(None, false);
        let use_generics = self.generate_generics(Some("'_"), false);
        let static_generics = self.generate_generics(Some("'static"), false);
        let trait_name = self.mode.name();
        let trait_function = self.mode.function();
        let as_ref = self.mode.as_ref();
        let doc = self.mode.doc();
        quote! {
            impl #def_generics #trait_name for #name #use_generics
            {
                type Owned = #name #static_generics;
                fn #trait_function(#as_ref self) -> Self::Owned {
                    #inner
                }
            }
            impl #def_generics #name #use_generics
            {
                #[doc=#doc]
                #[inline(always)]
                pub fn #trait_function(#as_ref self) -> #name #static_generics {
                    #trait_name::#trait_function(self)
                }
            }
        }
    }

    fn generate_generics(&self, lt_ident: Option<&str>, dedup: bool) -> Generics {
        let mut gen = Generics::default();
        let mut already_printed = false;

        for gp in self.input.generics.params.iter() {
            match gp {
                GenericParam::Lifetime(_) => {
                    if let Some(lt_ident) = lt_ident {
                        if !dedup || !already_printed {
                            gen.params.push(GenericParam::Lifetime(LifetimeParam::new(
                                Lifetime::new(lt_ident, Span::call_site()),
                            )));
                            already_printed = true;
                        }
                    }
                }
                GenericParam::Type(t) => {
                    abort!(t, "Generic types are supported (it's a todo)");
                }
                GenericParam::Const(c) => {
                    abort!(c, "Generic consts are supported (it's a todo)");
                }
            }
        }

        gen
    }
}
