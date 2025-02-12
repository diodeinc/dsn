#[cfg(test)]
mod tests {
    use chumsky::Parser;
    use insta::assert_debug_snapshot;
    use parser::Parsable;
    use parser_proc_macro::sexpr;

    #[sexpr(name = "test_struct")]
    struct TestUnitStruct;

    #[test]
    fn test_parse_unit_struct() {
        let parser = TestUnitStruct::parser();
        let result = parser.parse("test_struct");
        assert!(result.is_ok());
    }

    #[sexpr(name = "custom_name")]
    struct TestStructCustomName {}

    #[test]
    fn test_parse_custom_name() {
        let parser = TestStructCustomName::parser();
        let result = parser.parse("custom_name");
        assert!(result.is_ok());
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestStructFields {
        a: String,
        b: String,
    }

    #[test]
    fn test_parse_fields() {
        let parser = TestStructFields::parser();
        let result = parser.parse("(test_struct_fields hello world)");
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestStructFields {
            a: "hello",
            b: "world",
        }
        "###);
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestStructNamed {
        #[sexpr(name = "a")]
        a: String,
        #[sexpr(name = "b")]
        b: String,
    }

    #[test]
    fn test_parse_named() {
        let parser = TestStructNamed::parser();
        let result = parser.parse("(test_struct_named (a hello) (b world))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestStructNamed {
            a: "hello",
            b: "world",
        }
        "###);
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestUnnamed(String);

    #[test]
    fn test_parse_unnamed() {
        let parser = TestUnnamed::parser();
        let result = parser.parse("(test_unnamed hello)");
        assert!(result.is_ok());
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    enum TestEnum {
        A(String),
        B {
            #[sexpr(name = "data")]
            data: String,
        },
        C(),
        D,
    }

    #[test]
    fn test_parse_enum_case_unnamed() {
        let parser = TestEnum::parser();
        let result = parser.parse("(a hello)");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        A(
            "hello",
        )
        "###);
    }

    #[test]
    fn test_parse_enum_case_named() {
        let parser = TestEnum::parser();
        let result = parser.parse("(b (data world))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        B {
            data: "world",
        }
        "###);
    }

    #[test]
    fn test_parse_enum_case_unit() {
        let parser = TestEnum::parser();
        let result = parser.parse("c");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @"C");
    }

    #[test]
    fn test_parse_enum_case_empty() {
        let parser = TestEnum::parser();
        let result = parser.parse("d");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @"D");
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestOption {
        #[sexpr(name = "a")]
        a: Option<String>,
        #[sexpr(name = "b")]
        b: String,
    }

    #[test]
    fn test_parse_option() {
        let parser = TestOption::parser();
        let result = parser.parse("(test_option (a hello) (b world))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestOption {
            a: Some(
                "hello",
            ),
            b: "world",
        }
        "###);

        let parser = TestOption::parser();
        let result = parser.parse("(test_option (b world))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestOption {
            a: None,
            b: "world",
        }
        "###);
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestNestedOuter {
        #[sexpr(name = "a")]
        a: String,
        #[sexpr(name = "b")]
        b: TestNestedInner,
    }

    #[derive(Debug)]
    #[sexpr(anonymous)]
    #[allow(dead_code)]
    struct TestNestedInner {
        #[sexpr(name = "c")]
        c: String,
    }

    #[test]
    fn test_parse_nested() {
        let parser = TestNestedOuter::parser();
        let result = parser.parse("(test_nested_outer (a hello) (b (c world)))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestNestedOuter {
            a: "hello",
            b: TestNestedInner {
                c: "world",
            },
        }
        "###);
    }

    #[derive(Debug)]
    #[sexpr]
    #[allow(dead_code)]
    struct TestVec {
        #[sexpr(name = "items")]
        items: Vec<String>,
    }

    #[test]
    fn test_parse_vec() {
        let parser = TestVec::parser();
        let result = parser.parse("(test_vec (items hello world))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestVec {
            items: [
                "hello",
                "world",
            ],
        }
        "###);

        let parser = TestVec::parser();
        let result = parser.parse("(test_vec (items))");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        TestVec {
            items: [],
        }
        "###);
    }

    #[derive(Debug)]
    #[sexpr(anonymous)]
    enum TestEnumAnonymous {
        A,
        B,
        C,
    }

    #[test]
    fn test_parse_enum_anonymous() {
        let parser = TestEnumAnonymous::parser();
        let result = parser.parse("a");
        assert!(result.is_ok());
        assert_debug_snapshot!(result.unwrap(), @r###"
        A
        "###);
    }

    #[derive(Debug)]
    #[sexpr]
    enum TestEnumSingle {
        A,
    }

    #[test]
    fn test_parse_enum_single() {
        let parser = TestEnumSingle::parser();
        let result = parser.parse("a");
        assert!(result.is_ok());
    }

    #[derive(Debug)]
    #[sexpr]
    enum TestEnumEmpty {}

    #[test]
    fn test_parse_enum_empty() {
        let parser = TestEnumEmpty::parser();
        let result = parser.parse("");
        assert!(result.is_err());
    }
}
