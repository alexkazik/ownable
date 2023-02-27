use proc_macro2::Span;
use proc_macro_error::abort;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use syn::punctuated::Punctuated;
use syn::{spanned::Spanned, Attribute, Expr, ExprLit, Lit, Meta, MetaNameValue, Path, Token};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum Error<'a> {
    MetaParseError(syn::Error),
    DuplicateFeature,
    UnsupportedAttributeType,
    UnknownFeature,
    UnsupportedPath,
    ExpectedLiteral(&'a str),
    MissingLiteral(&'a str),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MetaParseError(e) => write!(f, "meta parse error: {e}"),
            Error::DuplicateFeature => write!(f, "duplicate feature"),
            Error::UnsupportedAttributeType => write!(f, "unsupported attribute type"),
            Error::UnknownFeature => write!(f, "unknown feature"),
            Error::UnsupportedPath => write!(f, "unsupported path"),
            Error::ExpectedLiteral(p) => write!(f, "expected literal: {p} (or no literal)"),
            Error::MissingLiteral(p) => write!(f, "missing literal: {p}"),
        }
    }
}

pub struct Features(HashMap<String, Param>);

impl Features {
    pub fn parse(name: &str, attrs: &[Attribute]) -> Self {
        let mut features = Self(HashMap::new());
        for a in attrs {
            if a.path().is_ident(name) {
                features.parse_meta(&a.meta);
            }
        }
        features
    }

    fn parse_meta(&mut self, meta: &Meta) {
        if let Meta::List(meta_list) = meta {
            let nested = meta_list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap_or_else(|e| abort!(meta_list.span(), Error::MetaParseError(e)));
            for outer in nested {
                let name;
                let lit;
                let span = outer.span();
                if let Meta::Path(path) = outer {
                    name = Self::path_to_name(&path);
                    lit = None;
                } else if let Meta::NameValue(MetaNameValue {
                    path,
                    value: Expr::Lit(ExprLit { lit: value, .. }),
                    ..
                }) = outer
                {
                    name = Self::path_to_name(&path);
                    lit = Some(value);
                } else {
                    abort!(outer.span(), Error::UnsupportedAttributeType);
                }

                if let Some(param) = self.0.insert(name, Param { span, lit }) {
                    abort!(param.span, Error::DuplicateFeature);
                }
            }
        } else {
            abort!(meta.span(), Error::UnsupportedAttributeType);
        }
    }

    fn path_to_name(path: &Path) -> String {
        path.get_ident()
            .unwrap_or_else(|| abort!(path.span(), Error::UnsupportedPath))
            .to_string()
    }

    pub fn get(&mut self, key: &str) -> Option<Param> {
        self.0.remove(key)
    }

    pub fn finish<T>(self, value: T) -> T {
        if let Some((_, params)) = self.0.into_iter().next() {
            abort!(params.span, Error::UnknownFeature);
        } else {
            value
        }
    }
}

pub struct Param {
    span: Span,
    lit: Option<Lit>,
}

impl Param {
    pub fn get_bool(&self) -> bool {
        if let Some(lit) = &self.lit {
            if let Lit::Bool(lit) = lit {
                lit.value
            } else {
                abort!(self.lit.span(), Error::ExpectedLiteral("bool"))
            }
        } else {
            true
        }
    }

    #[allow(dead_code)]
    pub fn opt_str(&self) -> Option<String> {
        if let Some(lit) = &self.lit {
            if let Lit::Str(lit) = lit {
                Some(lit.value())
            } else {
                abort!(self.lit.span(), Error::ExpectedLiteral("str"))
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn req_str(&self) -> String {
        self.opt_str()
            .unwrap_or_else(|| abort!(self.span, Error::MissingLiteral("str")))
    }
}
