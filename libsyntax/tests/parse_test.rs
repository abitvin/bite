use libsyntax::Parser;

#[test]
fn parse_the_grouped_7() {
    let parser = Parser::new();
    
    let result = parser.scan("7");
    assert!(result.is_err());

    let result = parser.scan("(7)");
    assert!(result.is_ok());

    let result = parser.scan("(8)");
    assert!(result.is_err());
}