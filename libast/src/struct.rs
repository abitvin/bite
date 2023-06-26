use crate::prop::Prop;

#[derive(Debug, PartialEq)]
pub struct Struct {
    id: String,
    props: Vec<Prop>,
}

impl Struct {
    pub fn new(id: impl Into<String>, props: Vec<Prop>) -> Self {
        Self { id: id.into(), props }
    }
}