use crate::attribute_parser::Features;
use syn::Attribute;

const ATTRIBUTE_NAME: &str = "ownable";

#[derive(Default)]
pub(crate) struct DeriveAttribute {
    pub(crate) clone: Option<bool>,
}

impl DeriveAttribute {
    #[must_use]
    pub(crate) fn new(attrs: &[Attribute]) -> Self {
        let mut features = Features::parse(ATTRIBUTE_NAME, attrs);
        let result = DeriveAttribute {
            clone: features.get("clone").map(|p| p.get_bool()),
        };
        features.finish(result)
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
