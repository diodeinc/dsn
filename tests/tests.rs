use chumsky::Parser;
use dsn::pcb::Pcb;
use insta::assert_debug_snapshot;
use parser::{Parsable, PrettyPrintError};
use std::fs;

macro_rules! test_dsn_file {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let input = fs::read_to_string(concat!("tests/inputs/", $path))
                .expect("failed to read test file");

            let pcb = Pcb::parser()
                .parse(input.as_str())
                .map_err(|e| {
                    for err in &e {
                        err.pretty_print(&input);
                    }
                    e
                })
                .expect("failed to parse pcb");

            assert_debug_snapshot!(pcb);
        }
    };
}

test_dsn_file!(test_rp2040, "rp2040.dsn");
