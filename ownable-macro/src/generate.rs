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
                "ToBorrowed can be only derived for a struct/enum with a lifetime"
            );
        }

        let lifetime_our = &Lifetime::new("'a", Span::call_site());
        let lifetime_placeholder = &Lifetime::new("'_", Span::call_site());
        let mut generics_definition = self.generate_generics(None);
        generics_definition
            .params
            .insert(0, LifetimeParam::new(lifetime_our.clone()).into());
        let generics_our = self.generate_generics(Some(lifetime_our));
        let generics_placeholder = self.generate_generics(Some(lifetime_placeholder));

        let name = &self.input.ident;
        let trait_name = Mode::ToBorrowed.name();
        let doc = Mode::ToBorrowed.doc();
        let function = if self.attribute.function {
            quote! {
                impl #generics_definition #name #generics_our
                {
                    #[doc=#doc]
                    #[inline(always)]
                    pub fn to_borrowed(&self) -> #name #generics_placeholder {
                        #trait_name::to_borrowed(self)
                    }
                }
            }
        } else {
            TokenStream::default()
        };

        quote! {
            impl #generics_definition #trait_name <#lifetime_our> for #name #generics_our
            {
                fn to_borrowed(&#lifetime_our self) -> Self {
                    #inner
                }
            }

            #function
        }
    }

    fn generate_mode_in_to_owned(&self, inner: &TokenStream) -> TokenStream {
        let lifetime_placeholder = &Lifetime::new("'_", Span::call_site());
        let lifetime_static = &Lifetime::new("'static", Span::call_site());
        let generics_definition = self.generate_generics(None);
        let generics_placeholder = self.generate_generics(Some(lifetime_placeholder));
        let generics_static = self.generate_generics(Some(lifetime_static));

        let name = &self.input.ident;
        let trait_name = self.mode.name();
        let trait_function = self.mode.function();
        let as_ref = self.mode.as_ref();
        let doc = self.mode.doc();
        let function = if self.attribute.function {
            quote! {
                impl #generics_definition #name #generics_placeholder
                {
                    #[doc=#doc]
                    #[inline(always)]
                    pub fn #trait_function(#as_ref self) -> #name #generics_static {
                        #trait_name::#trait_function(self)
                    }
                }
            }
        } else {
            TokenStream::default()
        };

        quote! {
            impl #generics_definition #trait_name for #name #generics_placeholder
            {
                type Owned = #name #generics_static;
                fn #trait_function(#as_ref self) -> Self::Owned {
                    #inner
                }
            }

            #function
        }
    }

    fn generate_generics(&self, lt: Option<&Lifetime>) -> Generics {
        let mut gen = Generics::default();

        for gp in self.input.generics.params.iter() {
            match gp {
                GenericParam::Lifetime(_) => {
                    if let Some(lt) = lt {
                        gen.params.push(LifetimeParam::new(lt.clone()).into());
                    }
                }
                GenericParam::Type(t) => {
                    abort!(t, "Generic types aren't supported (it's a todo)");
                }
                GenericParam::Const(c) => {
                    abort!(c, "Generic consts aren't supported (it's a todo)");
                }
            }
        }

        gen
    }
}
