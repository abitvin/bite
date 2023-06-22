use crate::scanner::Scanner;

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