use crate::{atoms, numeric, shapes};
use parser_proc_macro::Sexpr;

#[derive(Sexpr, Debug)]
pub struct Pcb {
    pub pcb_id: atoms::Id,
    pub parser: Option<ParserDescriptor>,
    // TODO: missing fields
    pub resolution: Option<ResolutionDescriptor>,
    // TODO: missing fields
    pub unit: Option<UnitDescriptor>,
    pub structure: Option<structure::StructureDescriptor>,
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "parser")]
pub struct ParserDescriptor {
    #[sexpr(name = "string_quote")]
    pub string_quote: Option<QuoteChar>,
    #[sexpr(name = "space_in_quoted_tokens")]
    pub space_in_quoted_tokens: atoms::Bool,
    #[sexpr(name = "host_cad")]
    pub host_cad: Option<atoms::Id>,
    #[sexpr(name = "host_version")]
    pub host_version: Option<atoms::Id>,
    pub constants: Vec<Constant>,
    // TODO: missing fields
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "resolution")]
pub struct ResolutionDescriptor {
    pub unit: numeric::DimensionUnit,
    pub value: numeric::PositiveInteger,
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "unit")]
pub struct UnitDescriptor {
    pub unit: numeric::DimensionUnit,
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "constant")]
pub struct Constant(pub atoms::Id, pub atoms::Id);

#[derive(Sexpr, Debug)]
pub enum QuoteChar {
    #[sexpr(name = "\"")]
    DoubleQuote,
    #[sexpr(name = "'")]
    SingleQuote,
    #[sexpr(name = "$")]
    DollarSign,
}

mod structure {
    use super::*;

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "structure")]
    pub struct StructureDescriptor {
        // TODO: missing fields
        pub layers: Vec<LayerDescriptor>,
        // TODO: missing fields
        pub boundaries: Vec<BoundaryDescriptor>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "layer")]
    pub struct LayerDescriptor {
        pub name: atoms::Id,
        #[sexpr(name = "type")]
        pub layer_type: LayerType,
        pub properties: Vec<UserPropertyDescriptor>,
        #[sexpr(name = "direction")]
        pub direction: Option<DirectionType>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    pub enum LayerType {
        Signal,
        Power,
        Mixed,
        Jumper,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "property")]
    pub struct UserPropertyDescriptor {
        pub descriptors: Vec<PropertyValueDescriptor>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "")]
    pub struct PropertyValueDescriptor {
        pub name: atoms::Id,
        pub value: PropertyValue,
    }

    #[derive(Sexpr, Debug)]
    pub enum PropertyValue {
        #[sexpr(anonymous)]
        Number(numeric::Number),
        #[sexpr(anonymous)]
        String(atoms::Id),
    }

    #[derive(Sexpr, Debug)]
    pub enum DirectionType {
        Horizontal,
        Vertical,
        Orthogonal,
        PositiveDiagonal,
        NegativeDiagonal,
        Diagonal,
        Off,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "boundary")]
    pub struct BoundaryDescriptor {
        pub boundary_type: BoundaryDescriptorType,
        pub rule_descriptor: Option<RuleDescriptor>,
    }

    #[derive(Sexpr, Debug)]
    pub enum BoundaryDescriptorType {
        #[sexpr(anonymous)]
        Paths(Vec<shapes::PathDescriptor>),
        #[sexpr(anonymous)]
        Rectangle(shapes::RectangleDescriptor),
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "rule")]
    pub struct RuleDescriptor {
        pub rule_type: Vec<RuleDescriptorType>,
    }

    #[derive(Sexpr, Debug)]
    pub enum RuleDescriptorType {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;
    use insta::assert_debug_snapshot;
    use parser::{Parsable, PrettyPrintError};

    #[test]
    fn test_pcb() {
        let input = r#"
(pcb "./build/expansion-board.dsn"
  (parser
    (string_quote ")
    (space_in_quoted_tokens on)
    (host_cad "KiCad's Pcbnew")
    (host_version "8.0.6")
  )
  (resolution um 10)
  (unit um)
  (structure
    (layer F.Cu
      (type signal)
      (property
        (index 0)
      )
    )
  )
)
    "#;

        let pcb = Pcb::parser()
            .parse(input)
            .map_err(|e| {
                for err in &e {
                    err.pretty_print(input);
                }
                e
            })
            .expect("failed to parse pcb");

        assert_debug_snapshot!(
            pcb,
            @r###"
        Pcb {
            pcb_id: Id(
                "./build/expansion-board.dsn",
            ),
            parser: Some(
                ParserDescriptor {
                    string_quote: Some(
                        DoubleQuote,
                    ),
                    space_in_quoted_tokens: Bool(
                        true,
                    ),
                    host_cad: Some(
                        Id(
                            "KiCad's Pcbnew",
                        ),
                    ),
                    host_version: Some(
                        Id(
                            "8.0.6",
                        ),
                    ),
                    constants: [],
                },
            ),
            resolution: Some(
                ResolutionDescriptor {
                    unit: Um,
                    value: PositiveInteger(
                        10,
                    ),
                },
            ),
            unit: Some(
                UnitDescriptor {
                    unit: Um,
                },
            ),
            structure: Some(
                StructureDescriptor {
                    layers: [
                        LayerDescriptor {
                            name: Id(
                                "F.Cu",
                            ),
                            layer_type: Signal,
                            properties: [
                                UserPropertyDescriptor {
                                    descriptors: [
                                        PropertyValueDescriptor {
                                            name: Id(
                                                "index",
                                            ),
                                            value: Number(
                                                Number {
                                                    sign: None,
                                                    number_type: PosInt(
                                                        PositiveInteger(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                            ],
                            direction: None,
                        },
                    ],
                    boundaries: [],
                },
            ),
        }
        "###
        );
    }
}
