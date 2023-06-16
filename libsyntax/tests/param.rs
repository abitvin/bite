use libsyntax::param::ParamParser;

#[test]
fn parse_parameter() {
    let parser = ParamParser::new();
    
    let result = parser.scan("nope");
    assert!(result.is_err());

    let result = parser.scan("nah:");
    assert!(result.is_err());

    let result = parser.scan(":no");
    assert!(result.is_err());

    let result = parser.scan("foo:yep");
    assert!(result.is_ok());

    let result = parser.scan("also   :  fine");
    assert!(result.is_ok());
}