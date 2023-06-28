use crate::prop::Prop;

#[derive(Debug, PartialEq)]
pub struct StructDecl {
    id: String,
    props: Vec<Prop>,
}

impl StructDecl {
    pub fn new(id: impl Into<String>, props: Vec<Prop>) -> Self {
        Self { id: id.into(), props }
    }
}