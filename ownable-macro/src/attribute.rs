use crate::attribute_parser::Features;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, abort_call_site};
use syn::{Attribute, DeriveInput, GenericParam, Generics, Lifetime};

const ATTRIBUTE_NAME: &str = "ownable";

#[derive(Default)]
pub(crate) struct DeriveAttribute {
    reference_lifetime: Vec<String>,
    pub(crate) clone: Option<bool>,
    pub(crate) function: bool,
}

impl DeriveAttribute {
    #[must_use]
    pub(crate) fn new(input: &DeriveInput) -> Self {
        let mut features = Features::parse(ATTRIBUTE_NAME, &input.attrs);
        let result = DeriveAttribute {
            reference_lifetime: features.get("reference").map_or_else(Vec::new, |p| {
                p.req_str()
                    .split(',')
                    .map(|l| {
                        let l = l.trim();
                        match l.strip_prefix('\'') {
                            None => abort!(p.span(), "not a lifetime or not used: \"{}\"", l),
                            Some(lt) => {
                                if Self::contains_lifetime(&input.generics, lt) {
                                    lt.to_string()
                                } else {
                                    abort!(p.span(), "not a lifetime or not used: \"{}\"", l)
                                }
                            }
                        }
                    })
                    .collect()
            }),
            clone: features.get("clone").map(|p| p.get_bool()),
            function: features.get("function").is_none_or(|p| p.get_bool()),
        };
        features.finish(result)
    }

    pub(crate) fn is_reference_lifetime(&self, ident: &Ident) -> bool {
        self.reference_lifetime.iter().any(|kl| ident == kl)
    }

    pub(crate) fn new_lifetime(&self) -> Lifetime {
        const TRY_LIFETIME: &str = "ownable";

        for l in TRY_LIFETIME
            .char_indices()
            .map(move |(pos, _)| &TRY_LIFETIME[..=pos])
        {
            if !self.reference_lifetime.iter().any(|kl| l == kl) {
                return Lifetime::new(&format!("'{l}"), Span::call_site());
            }
        }

        abort_call_site!("all of the following lifetimes are already used: 'o, 'ow, .. 'ownable");
    }

    fn contains_lifetime(generics: &Generics, lifetime: &str) -> bool {
        generics.params.iter().any(|p| match p {
            GenericParam::Lifetime(l) => l.lifetime.ident == lifetime,
            GenericParam::Type(_) | GenericParam::Const(_) => false,
        })
    }
}

#[derive(Default)]
pub(crate) struct FieldAttribute {
    pub(crate) clone: Option<bool>,
}

impl FieldAttribute {
    #[must_use]
    pub(crate) fn new(attrs: &[Attribute]) -> Self {
        let mut features = Features::parse(ATTRIBUTE_NAME, attrs);
        let result = FieldAttribute {
            clone: features.get("clone").map(|p| p.get_bool()),
        };
        features.finish(result)
    }
}

pub(crate) trait OrAssign<Rhs> {
    fn or_assign(&mut self, rhs: Rhs);
}

impl OrAssign<&FieldAttribute> for FieldAttribute {
    fn or_assign(&mut self, rhs: &FieldAttribute) {
        self.clone = self.clone.or(rhs.clone);
    }
}

impl OrAssign<&DeriveAttribute> for FieldAttribute {
    fn or_assign(&mut self, rhs: &DeriveAttribute) {
        self.clone = self.clone.or(rhs.clone);
    }
}
