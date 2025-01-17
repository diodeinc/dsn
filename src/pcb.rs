use crate::{atoms, numeric, shapes};
use chumsky::prelude::*;
use parser::Parsable;
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
    pub placement: Option<placement::PlacementDescriptor>,
    pub library: Option<library::LibraryDescriptor>,
    pub network: Option<network::NetworkDescriptor>,
    pub wiring: Option<wiring::WiringDescriptor>,
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
        // TODO: missing fields
        pub vias: ViaDescriptor,
        // TODO: missing fields
        pub rules: RuleDescriptor,
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
        pub descriptors: Vec<RuleDescriptorType>,
    }

    #[derive(Sexpr, Debug)]
    pub enum RuleDescriptorType {
        #[sexpr(anonymous)]
        Clearance(ClearanceDescriptor),
        // TODO: missing fields
        Width(numeric::PositiveDimension),
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "clearance")]
    pub struct ClearanceDescriptor {
        pub value: numeric::PositiveDimension,
        #[sexpr(name = "type")]
        pub types: Option<Vec<ClearanceType>>,
    }

    #[derive(Sexpr, Debug)]
    pub enum ClearanceType {
        SmdViaSameNet,
        ViaViaSameNet,
        BuriedViaGap {
            #[sexpr(name = "layer_depth")]
            layer_depth: numeric::PositiveInteger,
        },
        AntipadGap,
        PadToTurnGap,
        SmdToTurnGap,
        SmdSmd,
        // TODO: missing fields

        // This does not appear in the DSN spec, but it is used in the KiCad DSN files.
        DefaultSmd,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "via")]
    pub struct ViaDescriptor {
        pub padstack_ids: Vec<PadstackId>,
        pub spare_padstack_ids: Option<Vec<PadstackId>>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(anonymous)]
    pub struct PadstackId(pub atoms::Id);
}

mod placement {
    use super::*;

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "placement")]
    pub struct PlacementDescriptor {
        // TODO: missing fields
        pub components: Vec<ComponentInstance>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "component")]
    pub struct ComponentInstance {
        pub image_id: atoms::Id,
        pub placement: Vec<PlacementReference>,
    }

    #[derive(Sexpr, Debug, PartialEq)]
    #[sexpr(name = "place")]
    pub struct PlacementReference {
        pub component_id: atoms::Id,
        pub location: Option<PlacementReferenceLocation>,
        // TODO: missing fields
        #[sexpr(name = "PN")]
        pub part_number: Option<atoms::Id>,
    }

    #[derive(Sexpr, Debug, PartialEq)]
    #[sexpr(anonymous)]
    pub struct PlacementReferenceLocation {
        pub vertex: shapes::Vertex,
        pub side: Side,
        pub rotation: numeric::Number,
    }

    #[derive(Sexpr, Debug, PartialEq)]
    pub enum Side {
        Front,
        Back,
    }
}

mod library {
    use shapes::ShapeDescriptor;
    use structure::{ClearanceDescriptor, UserPropertyDescriptor};

    use super::*;

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "library")]
    pub struct LibraryDescriptor {
        pub unit: Option<UnitDescriptor>,
        pub images: Vec<ImageDescriptor>,
        pub jumpers: Option<Vec<JumperDescriptor>>,
        pub padstacks: Option<Vec<PadstackDescriptor>>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "image")]
    pub struct ImageDescriptor {
        pub image_id: atoms::Id,
        pub side: Option<Side>,
        pub unit: Option<UnitDescriptor>,
        // The spec seems to indicate this should be Option<OutlineDescriptor>, but that doesn't
        // seem to be the case in practice.
        pub outlines: Vec<OutlineDescriptor>,
        pub pins: Vec<PinDescriptor>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    pub enum Side {
        Front,
        Back,
        Both,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "outline")]
    pub struct OutlineDescriptor {
        #[sexpr(anonymous)]
        pub shape: shapes::ShapeDescriptor,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "pin")]
    pub struct PinDescriptor {
        pub padstack_id: structure::PadstackId,
        #[sexpr(name = "rotate")]
        pub rotate: Option<numeric::Number>,
        pub reference: PinRefDescriptor,
        pub property: Option<UserPropertyDescriptor>,
    }

    #[derive(Sexpr, Debug)]
    pub enum PinRefDescriptor {
        #[sexpr(anonymous)]
        ReferenceDescriptor {
            pin_id: atoms::Id,
            vertex: shapes::Vertex,
        },
        #[sexpr(name = "array")]
        PinArrayDescriptor {
            begin_index: numeric::PositiveInteger,
            end_index: numeric::PositiveInteger,
            index_step: numeric::PositiveInteger,
            x0: numeric::Number,
            y0: numeric::Number,
            xstep: numeric::Number,
            ystep: numeric::Number,
            #[sexpr(name = "prefix")]
            pin_prefix_id: Option<atoms::Id>,
            #[sexpr(name = "suffix")]
            pin_suffix_id: Option<atoms::Id>,
        },
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "jumper")]
    pub struct JumperDescriptor {
        #[sexpr(name = "length")]
        pub length: numeric::PositiveDimension,
        #[sexpr(name = "use_via")]
        pub use_via: Option<structure::PadstackId>,
        #[sexpr(name = "height")]
        pub height: Option<numeric::Number>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "padstack")]
    pub struct PadstackDescriptor {
        pub padstack_id: structure::PadstackId,
        pub unit: Option<UnitDescriptor>,
        pub shapes: Vec<PadstackShapeDescriptor>,
        #[sexpr(name = "attach")]
        pub attach: Option<AttachDescriptor>,
        pub pad_via_sites: Option<Vec<PadViaSiteDescriptor>>,
        #[sexpr(name = "rotate")]
        pub rotate: Option<atoms::Bool>,
        #[sexpr(name = "absolute")]
        pub absolute: Option<atoms::Bool>,
        pub rule: Option<ClearanceDescriptor>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "shape")]
    pub struct PadstackShapeDescriptor {
        pub shape: ShapeDescriptor,
        #[sexpr(name = "reduced")]
        pub reduced_shape: Option<ShapeDescriptor>,
        pub connect: Option<atoms::Bool>,
        pub windows: Option<Vec<WindowDescriptor>>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "window")]
    pub struct WindowDescriptor {
        pub shape: ShapeDescriptor,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "attach")]
    pub enum AttachDescriptor {
        Off,
        On {
            #[sexpr(name = "use_via")]
            use_via: Option<atoms::Id>,
        },
        #[sexpr(anonymous)]
        None,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "via_site")]
    pub enum PadViaSiteDescriptor {
        #[sexpr(anonymous)]
        Vertex(shapes::Vertex),
        Off,
    }
}

mod network {
    use structure::RuleDescriptor;

    use super::*;

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "network")]
    pub struct NetworkDescriptor {
        pub net: Vec<NetDescriptor>,
        pub classes: Option<Vec<ClassDescriptor>>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "net")]
    pub struct NetDescriptor {
        pub net_id: atoms::Id,
        #[sexpr(name = "unassigned")]
        pub unassigned: Option<()>,
        #[sexpr(name = "net_number")]
        pub net_number: Option<numeric::Number>,
        pub pins_or_order: Option<NetPinsOrOrder>,
    }

    #[derive(Debug)]
    pub struct PinReference {
        pub component_id: atoms::Id,
        pub pin_id: atoms::Id,
    }

    impl Parsable for PinReference {
        fn parser() -> impl chumsky::Parser<char, PinReference, Error = Simple<char>> {
            // For some reason I can't get this to work with atoms::Id::parser(), so falling back
            // to a simpler strategy. This is less expressive than it should be.
            let id_parser = filter(|c: &char| c.is_ascii_alphanumeric()).repeated();

            id_parser
                .then_ignore(just("-"))
                .then(id_parser)
                .map(|(component_id, pin_id)| Self {
                    component_id: component_id.into_iter().collect::<String>().into(),
                    pin_id: pin_id.into_iter().collect::<String>().into(),
                })
                .padded()
        }
    }

    #[derive(Sexpr, Debug)]
    pub enum NetPinsOrOrder {
        Pins(Vec<PinReference>),
        Order(Vec<PinReference>),
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "class")]
    pub struct ClassDescriptor {
        pub class_id: atoms::Id,
        pub net_or_composite: Vec<NetOrCompositeList>,
        pub circuit: CircuitDescriptor,
        pub rules: Option<RuleDescriptor>,
        pub layer_rules: Option<Vec<LayerRuleDescriptor>>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    pub enum NetOrCompositeList {
        #[sexpr(anonymous)]
        NetId(atoms::Id),
        #[sexpr(anonymous)]
        CompositeNameList(CompositeNameList),
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "composite")]
    pub struct CompositeNameList {
        pub prefix: Option<atoms::Id>,
        pub begin_index: numeric::PositiveInteger,
        pub end_index: numeric::PositiveInteger,
        pub step: numeric::PositiveInteger,
        pub suffix: Option<atoms::Id>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "circuit")]
    pub struct CircuitDescriptor {
        pub descriptors: Vec<CircuitDescriptorType>,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "layer_rule")]
    pub struct LayerRuleDescriptor {
        pub layer_ids: Vec<atoms::Id>,
        pub rule: structure::RuleDescriptor,
    }

    #[derive(Sexpr, Debug)]
    pub enum CircuitDescriptorType {
        // TODO: missing types
        UseVia {
            padstack_ids: Vec<structure::PadstackId>,
        },
    }
}

mod wiring {
    use super::*;

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "wiring")]
    pub struct WiringDescriptor {
        // TODO: missing fields
        pub wires: Vec<WireDescriptor>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    pub enum WireDescriptor {
        #[sexpr(anonymous)]
        Shape(WireShapeDescriptor),
        #[sexpr(anonymous)]
        Via(WireViaDescriptor),
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "wire")]
    pub struct WireShapeDescriptor {
        pub shape: shapes::ShapeDescriptor,
        #[sexpr(name = "net")]
        pub net_id: Option<atoms::Id>,
        #[sexpr(name = "type")]
        pub wire_type: Option<WireType>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug)]
    pub enum WireType {
        Fix,
        Route,
        Normal,
        Protect,
    }

    #[derive(Sexpr, Debug)]
    #[sexpr(name = "via")]
    pub struct WireViaDescriptor {
        pub padstack_id: structure::PadstackId,
        pub vertices: Vec<shapes::Vertex>,
        #[sexpr(name = "net")]
        pub net: Option<atoms::Id>,
        #[sexpr(name = "via_number")]
        pub via_number: Option<numeric::PositiveInteger>,
        #[sexpr(name = "type")]
        pub wire_type: Option<WireType>,
        // TODO: missing fields
    }
}
