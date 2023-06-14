use libsyntax::parser_grouped_7;

#[test]
fn parse_the_grouped_7() {
    let parser = parser_grouped_7();
    
    let result = parser.scan("grouped", "7");
    assert!(result.is_err());

    let result = parser.scan("grouped", "(7)");
    assert!(result.is_ok());

    let result = parser.scan("grouped", "(8)");
    assert!(result.is_err());
}