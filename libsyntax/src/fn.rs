use crate::param::Param;

pub struct FnDecl {
    id: String,
    params: Vec<Param>,
}

impl FnDecl {
    fn from_id(value: &str) -> Self {
        Self { id: String::from(value), params: vec![] }
    }

    fn from_params(value: Vec<Param>) -> Self {
        Self { id: String::new(), params: value }
    }
}