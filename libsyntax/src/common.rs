use crate::scanner::Scanner;

pub fn parse_int(ctx: &mut Scanner) -> Option<String> {
    let mut c = ctx.scan("-")
        .unwrap_or(String::new());

    unimplemented!()
}

pub fn parse_id(ctx: &mut Scanner) -> Option<String> {
    let mut c = ctx.scan_alphabetic()
        .map(|c| String::from(c))?;
    
    loop {
        let c2 = ctx.scan_alphanumeric();
        
        if let Some(c2) = c2 {
            c = c + &String::from(c2);
        }
        else {
            break;
        }
    }
    
    Some(c)
}