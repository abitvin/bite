use crate::{common::parse_id, scanner::{Parse, Scanner}};
use libast::{prop::Prop, struct_decl::StructDecl};

impl Parse for StructDecl {
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
        
        Some(Self::new(id, props))
    }
}

fn parse_prop(scn: &mut Scanner) -> Option<Prop> {
    scn.skip_spaces();
    let prop = Prop::parse(scn)?;
    scn.skip_spaces();
    scn.skip_newline();
    Some(prop)
}