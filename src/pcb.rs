use crate::{atoms, numeric, shapes};
use chumsky::prelude::*;
use parser::Parsable;
use parser_proc_macro::Sexpr;
use pyo3::prelude::*;

#[derive(Sexpr, Debug, Clone)]
#[pyclass]
pub struct Pcb {
    #[pyo3(get, set)]
    pub pcb_id: atoms::Id,
    #[pyo3(get, set)]
    pub parser: Option<ParserDescriptor>,
    // TODO: missing fields
    #[pyo3(get, set)]
    pub resolution: Option<ResolutionDescriptor>,
    // TODO: missing fields
    #[pyo3(get, set)]
    pub unit: Option<UnitDescriptor>,
    #[pyo3(get, set)]
    pub structure: Option<structure::StructureDescriptor>,
    #[pyo3(get, set)]
    pub placement: Option<placement::PlacementDescriptor>,
    #[pyo3(get, set)]
    pub library: Option<library::LibraryDescriptor>,
    #[pyo3(get, set)]
    pub network: Option<network::NetworkDescriptor>,
    #[pyo3(get, set)]
    pub wiring: Option<wiring::WiringDescriptor>,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "parser")]
#[pyclass]
pub struct ParserDescriptor {
    #[pyo3(get, set)]
    #[sexpr(name = "string_quote")]
    pub string_quote: Option<QuoteChar>,
    #[pyo3(get, set)]
    #[sexpr(name = "space_in_quoted_tokens")]
    pub space_in_quoted_tokens: atoms::Bool,
    #[pyo3(get, set)]
    #[sexpr(name = "host_cad")]
    pub host_cad: Option<atoms::Id>,
    #[pyo3(get, set)]
    #[sexpr(name = "host_version")]
    pub host_version: Option<atoms::Id>,
    #[pyo3(get, set)]
    pub constants: Vec<Constant>,
    // TODO: missing fields
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "resolution")]
#[pyclass]
pub struct ResolutionDescriptor {
    #[pyo3(get, set)]
    pub unit: numeric::DimensionUnit,
    #[pyo3(get, set)]
    pub value: numeric::PositiveInteger,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "unit")]
#[pyclass]
pub struct UnitDescriptor {
    #[pyo3(get, set)]
    pub unit: numeric::DimensionUnit,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "constant")]
#[pyclass]
pub struct Constant(
    #[pyo3(get, set, name = "first")] pub atoms::Id,
    #[pyo3(get, set, name = "second")] pub atoms::Id,
);

#[derive(Sexpr, Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum QuoteChar {
    #[sexpr(name = "\"")]
    DoubleQuote,
    #[sexpr(name = "'")]
    SingleQuote,
    #[sexpr(name = "$")]
    DollarSign,
}

#[pymodule]
mod structure {
    use super::*;

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "structure")]
    #[pyclass]
    pub struct StructureDescriptor {
        // TODO: missing fields
        #[pyo3(get, set)]
        pub layers: Vec<LayerDescriptor>,
        // TODO: missing fields
        #[pyo3(get, set)]
        pub boundaries: Vec<BoundaryDescriptor>,
        // TODO: missing fields
        #[pyo3(get, set)]
        pub vias: ViaDescriptor,
        // TODO: missing fields
        #[pyo3(get, set)]
        pub rules: RuleDescriptor,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "layer")]
    #[pyclass]
    pub struct LayerDescriptor {
        #[pyo3(get, set)]
        pub name: atoms::Id,
        #[pyo3(get, set)]
        #[sexpr(name = "type")]
        pub layer_type: LayerType,
        #[pyo3(get, set)]
        pub properties: Vec<UserPropertyDescriptor>,
        #[pyo3(get, set)]
        #[sexpr(name = "direction")]
        pub direction: Option<DirectionType>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone, PartialEq)]
    #[pyclass(eq, eq_int)]
    pub enum LayerType {
        Signal,
        Power,
        Mixed,
        Jumper,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "property")]
    #[pyclass]
    pub struct UserPropertyDescriptor {
        #[pyo3(get, set)]
        pub descriptors: Vec<PropertyValueDescriptor>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "")]
    #[pyclass]
    pub struct PropertyValueDescriptor {
        #[pyo3(get, set)]
        pub name: atoms::Id,
        #[pyo3(get, set)]
        pub value: PropertyValue,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum PropertyValue {
        #[sexpr(anonymous)]
        Number(numeric::Number),
        #[sexpr(anonymous)]
        String(atoms::Id),
    }

    #[derive(Sexpr, Debug, Clone, PartialEq)]
    #[pyclass(eq, eq_int)]
    pub enum DirectionType {
        Horizontal,
        Vertical,
        Orthogonal,
        PositiveDiagonal,
        NegativeDiagonal,
        Diagonal,
        Off,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "boundary")]
    #[pyclass]
    pub struct BoundaryDescriptor {
        #[pyo3(get, set)]
        pub boundary_type: BoundaryDescriptorType,
        #[pyo3(get, set)]
        pub rule_descriptor: Option<RuleDescriptor>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum BoundaryDescriptorType {
        #[sexpr(anonymous)]
        Paths(Vec<shapes::PathDescriptor>),
        #[sexpr(anonymous)]
        Rectangle(shapes::RectangleDescriptor),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "rule")]
    #[pyclass]
    pub struct RuleDescriptor {
        #[pyo3(get, set)]
        pub descriptors: Vec<RuleDescriptorType>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum RuleDescriptorType {
        #[sexpr(anonymous)]
        Clearance(ClearanceDescriptor),
        // TODO: missing fields
        Width(numeric::PositiveDimension),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "clearance")]
    #[pyclass]
    pub struct ClearanceDescriptor {
        #[pyo3(get, set)]
        pub value: numeric::PositiveDimension,
        #[pyo3(get, set)]
        #[sexpr(name = "type")]
        pub types: Option<Vec<ClearanceType>>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum ClearanceType {
        SmdViaSameNet(),
        ViaViaSameNet(),
        BuriedViaGap {
            #[sexpr(name = "layer_depth")]
            layer_depth: numeric::PositiveInteger,
        },
        AntipadGap(),
        PadToTurnGap(),
        SmdToTurnGap(),
        SmdSmd(),
        // TODO: missing fields

        // This does not appear in the DSN spec, but it is used in the KiCad DSN files.
        DefaultSmd(),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "via")]
    #[pyclass]
    pub struct ViaDescriptor {
        #[pyo3(get, set)]
        pub padstack_ids: Vec<PadstackId>,
        #[pyo3(get, set)]
        pub spare_padstack_ids: Option<Vec<PadstackId>>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(anonymous)]
    #[pyclass]
    pub struct PadstackId(pub atoms::Id);

    #[pymethods]
    impl PadstackId {
        fn __str__(&self) -> PyResult<String> {
            Ok(self.0.to_string())
        }
    }
}

#[pymodule]
mod placement {
    use super::*;

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "placement")]
    #[pyclass]
    pub struct PlacementDescriptor {
        // TODO: missing fields
        #[pyo3(get, set)]
        pub components: Vec<ComponentInstance>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "component")]
    #[pyclass]
    pub struct ComponentInstance {
        #[pyo3(get, set)]
        pub image_id: atoms::Id,
        #[pyo3(get, set)]
        pub placement: Vec<PlacementReference>,
    }

    #[derive(Sexpr, Debug, PartialEq, Clone)]
    #[sexpr(name = "place")]
    #[pyclass]
    pub struct PlacementReference {
        #[pyo3(get, set)]
        pub component_id: atoms::Id,
        #[pyo3(get, set)]
        pub location: Option<PlacementReferenceLocation>,
        // TODO: missing fields
        #[pyo3(get, set)]
        #[sexpr(name = "PN")]
        pub part_number: Option<atoms::Id>,
    }

    #[derive(Sexpr, Debug, PartialEq, Clone)]
    #[sexpr(anonymous)]
    #[pyclass]
    pub struct PlacementReferenceLocation {
        #[pyo3(get, set)]
        pub vertex: shapes::Vertex,
        #[pyo3(get, set)]
        pub side: Side,
        #[pyo3(get, set)]
        pub rotation: numeric::Number,
    }

    #[derive(Sexpr, Debug, PartialEq, Clone)]
    #[pyclass(eq, eq_int)]
    pub enum Side {
        Front,
        Back,
    }
}

#[pymodule]
mod library {
    use shapes::ShapeDescriptor;
    use structure::{ClearanceDescriptor, UserPropertyDescriptor};

    use super::*;

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    #[sexpr(name = "library")]
    pub struct LibraryDescriptor {
        #[pyo3(get, set)]
        pub unit: Option<UnitDescriptor>,
        #[pyo3(get, set)]
        pub images: Vec<ImageDescriptor>,
        #[pyo3(get, set)]
        pub jumpers: Option<Vec<JumperDescriptor>>,
        #[pyo3(get, set)]
        pub padstacks: Option<Vec<PadstackDescriptor>>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "image")]
    #[pyclass]
    pub struct ImageDescriptor {
        #[pyo3(get, set)]
        pub image_id: atoms::Id,
        #[pyo3(get, set)]
        pub side: Option<Side>,
        #[pyo3(get, set)]
        pub unit: Option<UnitDescriptor>,
        // The spec seems to indicate this should be Option<OutlineDescriptor>, but that doesn't
        // seem to be the case in practice.
        #[pyo3(get, set)]
        pub outlines: Vec<OutlineDescriptor>,
        #[pyo3(get, set)]
        pub pins: Vec<PinDescriptor>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone, PartialEq)]
    #[pyclass(eq, eq_int)]
    pub enum Side {
        Front,
        Back,
        Both,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "outline")]
    #[pyclass]
    pub struct OutlineDescriptor {
        #[pyo3(get, set)]
        pub shape: shapes::ShapeDescriptor,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "pin")]
    #[pyclass]
    pub struct PinDescriptor {
        #[pyo3(get, set)]
        pub padstack_id: structure::PadstackId,
        #[pyo3(get, set)]
        #[sexpr(name = "rotate")]
        pub rotate: Option<numeric::Number>,
        #[pyo3(get, set)]
        pub reference: PinRefDescriptor,
        #[pyo3(get, set)]
        pub property: Option<UserPropertyDescriptor>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
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

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    #[sexpr(name = "jumper")]
    pub struct JumperDescriptor {
        #[pyo3(get, set)]
        #[sexpr(name = "length")]
        pub length: numeric::PositiveDimension,
        #[pyo3(get, set)]
        #[sexpr(name = "use_via")]
        pub use_via: Option<structure::PadstackId>,
        #[pyo3(get, set)]
        #[sexpr(name = "height")]
        pub height: Option<numeric::Number>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "padstack")]
    #[pyclass]
    pub struct PadstackDescriptor {
        #[pyo3(get, set)]
        pub padstack_id: structure::PadstackId,
        #[pyo3(get, set)]
        pub unit: Option<UnitDescriptor>,
        #[pyo3(get, set)]
        pub shapes: Vec<PadstackShapeDescriptor>,
        #[pyo3(get, set)]
        #[sexpr(name = "attach")]
        pub attach: Option<AttachDescriptor>,
        #[pyo3(get, set)]
        pub pad_via_sites: Option<Vec<PadViaSiteDescriptor>>,
        #[pyo3(get, set)]
        #[sexpr(name = "rotate")]
        pub rotate: Option<atoms::Bool>,
        #[pyo3(get, set)]
        #[sexpr(name = "absolute")]
        pub absolute: Option<atoms::Bool>,
        #[pyo3(get, set)]
        pub rule: Option<ClearanceDescriptor>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "shape")]
    #[pyclass]
    pub struct PadstackShapeDescriptor {
        #[pyo3(get, set)]
        pub shape: ShapeDescriptor,
        #[pyo3(get, set)]
        #[sexpr(name = "reduced")]
        pub reduced_shape: Option<ShapeDescriptor>,
        #[pyo3(get, set)]
        pub connect: Option<atoms::Bool>,
        #[pyo3(get, set)]
        pub windows: Option<Vec<WindowDescriptor>>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    #[sexpr(name = "window")]
    pub struct WindowDescriptor {
        #[pyo3(get, set)]
        pub shape: ShapeDescriptor,
    }

    #[derive(Sexpr, Debug, Clone, PartialEq)]
    #[pyclass]
    #[sexpr(name = "attach")]
    pub enum AttachDescriptor {
        Off(),
        On {
            #[sexpr(name = "use_via")]
            use_via: Option<atoms::Id>,
        },
        #[sexpr(anonymous)]
        None(),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "via_site")]
    #[pyclass]
    pub enum PadViaSiteDescriptor {
        #[sexpr(anonymous)]
        Vertex(shapes::Vertex),
        Off(),
    }
}

#[pymodule]
mod network {
    use structure::RuleDescriptor;

    use super::*;

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "network")]
    #[pyclass]
    pub struct NetworkDescriptor {
        #[pyo3(get, set)]
        pub net: Vec<NetDescriptor>,
        #[pyo3(get, set)]
        pub classes: Option<Vec<ClassDescriptor>>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "net")]
    #[pyclass]
    pub struct NetDescriptor {
        #[pyo3(get, set)]
        pub net_id: atoms::Id,
        #[sexpr(name = "unassigned")]
        pub unassigned: Option<()>,
        #[pyo3(get, set)]
        #[sexpr(name = "net_number")]
        pub net_number: Option<numeric::Number>,
        #[pyo3(get, set)]
        pub pins_or_order: Option<NetPinsOrOrder>,
    }

    #[derive(Debug, Clone)]
    #[pyclass]
    pub struct PinReference {
        #[pyo3(get, set)]
        pub component_id: atoms::Id,
        #[pyo3(get, set)]
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

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum NetPinsOrOrder {
        Pins(Vec<PinReference>),
        Order(Vec<PinReference>),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "class")]
    #[pyclass]
    pub struct ClassDescriptor {
        #[pyo3(get, set)]
        pub class_id: atoms::Id,
        #[pyo3(get, set)]
        pub net_or_composite: Vec<NetOrCompositeList>,
        #[pyo3(get, set)]
        pub circuit: CircuitDescriptor,
        #[pyo3(get, set)]
        pub rules: Option<RuleDescriptor>,
        #[pyo3(get, set)]
        pub layer_rules: Option<Vec<LayerRuleDescriptor>>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum NetOrCompositeList {
        #[sexpr(anonymous)]
        NetId(atoms::Id),
        #[sexpr(anonymous)]
        CompositeNameList(CompositeNameList),
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "composite")]
    #[pyclass]
    pub struct CompositeNameList {
        #[pyo3(get, set)]
        pub prefix: Option<atoms::Id>,
        #[pyo3(get, set)]
        pub begin_index: numeric::PositiveInteger,
        #[pyo3(get, set)]
        pub end_index: numeric::PositiveInteger,
        #[pyo3(get, set)]
        pub step: numeric::PositiveInteger,
        #[pyo3(get, set)]
        pub suffix: Option<atoms::Id>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "circuit")]
    #[pyclass]
    pub struct CircuitDescriptor {
        #[pyo3(get, set)]
        pub descriptors: Vec<CircuitDescriptorType>,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "layer_rule")]
    #[pyclass]
    pub struct LayerRuleDescriptor {
        #[pyo3(get, set)]
        pub layer_ids: Vec<atoms::Id>,
        #[pyo3(get, set)]
        pub rule: structure::RuleDescriptor,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum CircuitDescriptorType {
        // TODO: missing types
        UseVia {
            padstack_ids: Vec<structure::PadstackId>,
        },
    }
}

#[pymodule]
mod wiring {
    use super::*;

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "wiring")]
    #[pyclass]
    pub struct WiringDescriptor {
        // TODO: missing fields
        #[pyo3(get, set)]
        pub wires: Vec<WireDescriptor>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone)]
    #[pyclass]
    pub enum WireDescriptor {
        #[sexpr(anonymous)]
        Shape(WireShapeDescriptor),
        #[sexpr(anonymous)]
        Via(WireViaDescriptor),
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "wire")]
    #[pyclass]
    pub struct WireShapeDescriptor {
        #[pyo3(get, set)]
        pub shape: shapes::ShapeDescriptor,
        #[pyo3(get, set)]
        #[sexpr(name = "net")]
        pub net_id: Option<atoms::Id>,
        #[pyo3(get, set)]
        #[sexpr(name = "type")]
        pub wire_type: Option<WireType>,
        // TODO: missing fields
    }

    #[derive(Sexpr, Debug, Clone, PartialEq)]
    #[pyclass(eq, eq_int)]
    pub enum WireType {
        Fix,
        Route,
        Normal,
        Protect,
    }

    #[derive(Sexpr, Debug, Clone)]
    #[sexpr(name = "via")]
    #[pyclass]
    pub struct WireViaDescriptor {
        #[pyo3(get, set)]
        pub padstack_id: structure::PadstackId,
        #[pyo3(get, set)]
        pub vertices: Vec<shapes::Vertex>,
        #[pyo3(get, set)]
        #[sexpr(name = "net")]
        pub net: Option<atoms::Id>,
        #[pyo3(get, set)]
        #[sexpr(name = "via_number")]
        pub via_number: Option<numeric::PositiveInteger>,
        #[pyo3(get, set)]
        #[sexpr(name = "type")]
        pub wire_type: Option<WireType>,
        // TODO: missing fields
    }
}
