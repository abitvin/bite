use crate::{common::parse_id, prop::Prop, scanner::{Parse, Scanner}};

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

impl Parse for Struct {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn)?;
        scn.skip_spaces();
        scn.scan(":")?;
        scn.skip_spaces();
        scn.scan("struct")?;
        scn.skip_spaces();
        scn.skip_newline();

        let mut props = vec![];

        loop {
            let mut try_scn = scn.clone();
            let prop = parse_prop(&mut try_scn);

            if let Some(prop) = prop {
                scn.replace(try_scn);
                props.push(prop);
            }
            else {
                break;
            }
        }

        scn.skip_spaces();
        scn.scan(".")?;
        
        Some(Self { id, props })
    }
}

fn parse_prop(scn: &mut Scanner) -> Option<Prop> {
    scn.skip_spaces();
    let prop = Prop::parse(scn)?;
    scn.skip_spaces();
    scn.skip_newline();
    Some(prop)
}