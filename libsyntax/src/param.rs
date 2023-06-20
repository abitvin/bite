pub struct Param {
    pub id: String,
    pub typ: String,
}

impl Param {
    fn new(id: &str, typ: &str) -> Self {
        Self { 
            id: String::from(id), 
            typ: String::from(typ),
        }
    }
}