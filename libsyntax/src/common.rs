use crate::scanner::Scanner;

pub fn parse_bool(scn: &mut Scanner) -> Option<bool> {
    let mut try_scn = scn.clone();
    
    if try_scn.scan("true").is_some() {
        scn.replace(try_scn);
        Some(true)
    } else if try_scn.scan("false").is_some() { 
        scn.replace(try_scn);
        Some(false)
    } else {
        None
    }
}

pub fn parse_int(scn: &mut Scanner) -> Option<String> {
    let c = scn.scan("-")
        .unwrap_or(String::new());

    let digits = scn.scan_digits()?;
    Some(c + &digits)
}

pub fn parse_id(scn: &mut Scanner) -> Option<String> {
    let mut c = scn.scan_alphabetic()
        .map(String::from)?;
    
    loop {
        let c2 = scn.scan_alphanumeric();
        
        if let Some(c2) = c2 {
            c = c + &String::from(c2);
        }
        else {
            break;
        }
    }
    
    Some(c)
}

pub fn parse_uint(scn: &mut Scanner) -> Option<String> {
    scn.scan_digits()
}

pub fn parse_two_ids(scn: &mut Scanner) -> Option<(String, String)> {
    let id = parse_id(scn)?;
    scn.skip_spaces();
    scn.scan(":")?;
    scn.skip_spaces();
    let typ = parse_id(scn)?;
    
    Some((id, typ ))
}

pub fn try_parse<T>(scn: &mut Scanner, f: impl Fn(&mut Scanner) -> Option<T>) -> Option<T> {
    let mut try_scn = scn.clone();

    if let Some(x) = f(&mut try_scn) {
        scn.replace(try_scn);
        Some(x)
    }
    else {
        None
    }
}