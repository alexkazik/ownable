use crate::mode::Mode;
use crate::Derive;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use std::iter::once;
use syn::token::{Colon, Where};
use syn::{
    GenericParam, Generics, Lifetime, LifetimeParam, Path, PathSegment, PredicateType, Type,
    TypeParam, TypeParamBound, TypePath, WhereClause, WherePredicate,
};

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
        let generics_where = self.generate_where(lifetime_our);

        let name = &self.input.ident;
        let trait_name = Mode::ToBorrowed.name();
        let doc = Mode::ToBorrowed.doc();
        let function = if self.attribute.function {
            quote! {
                impl #generics_definition #name #generics_our #generics_where
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
            impl #generics_definition #trait_name <#lifetime_our> for #name #generics_our #generics_where
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
        let generics_where = self.generate_where(lifetime_static);

        let name = &self.input.ident;
        let trait_name = self.mode.name();
        let trait_function = self.mode.function();
        let as_ref = self.mode.as_ref();
        let doc = self.mode.doc();
        let function = if self.attribute.function {
            quote! {
                impl #generics_definition #name #generics_placeholder #generics_where
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
            impl #generics_definition #trait_name for #name #generics_placeholder #generics_where
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
                    gen.params.push(TypeParam::from(t.ident.clone()).into());
                }
                GenericParam::Const(c) => {
                    abort!(c, "Generic consts aren't supported (it's a todo)");
                }
            }
        }

        gen
    }

    fn generate_where(&self, lt: &Lifetime) -> WhereClause {
        let mut w = Vec::new();

        for gp in self.input.generics.params.iter() {
            match gp {
                GenericParam::Lifetime(_) => {}
                GenericParam::Type(t) => {
                    w.push(WherePredicate::Type(PredicateType {
                        lifetimes: None,
                        bounded_ty: Type::Path(TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: once(PathSegment::from(t.ident.clone())).collect(),
                            },
                        }),
                        colon_token: Colon::default(),
                        bounds: t.bounds.iter().cloned().collect(),
                    }));
                }
                GenericParam::Const(c) => {
                    abort!(c, "Generic consts are supported (it's a todo)");
                }
            }
        }

        if let Some(wc) = &self.input.generics.where_clause {
            w.extend(wc.predicates.iter().cloned());
        }

        WhereClause {
            where_token: Where::default(),
            predicates: w
                .into_iter()
                .filter_map(|wp| self.set_lifetime(lt, wp))
                .collect(),
        }
    }

    #[allow(clippy::unused_self)]
    fn set_lifetime(&self, lt: &Lifetime, wp: WherePredicate) -> Option<WherePredicate> {
        match wp {
            WherePredicate::Lifetime(_) => None,
            WherePredicate::Type(mut pt) => {
                pt.lifetimes = None;
                pt.bounds = pt
                    .bounds
                    .into_iter()
                    .map(|tpb| match tpb {
                        TypeParamBound::Lifetime(_) => TypeParamBound::Lifetime(lt.clone()),
                        t => t,
                    })
                    .collect();
                Some(WherePredicate::Type(pt))
            }
            // When stable, enable: #[cfg_attr(test, deny(non_exhaustive_omitted_patterns))]
            _ => abort_call_site!("Unsupported WherePredicate"),
        }
    }
}
