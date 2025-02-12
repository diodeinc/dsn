use crate::{atoms, numeric, pcb};
use chumsky::prelude::*;
use parser::Parsable;
use parser_proc_macro::sexpr;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[sexpr(name = "session")]
#[pyclass]
pub struct Session {
    pub session_id: atoms::Id,
    #[sexpr(name = "base_design")]
    pub base_design: atoms::Id,
    // TODO: missing fields
    pub routes: RouteDescriptor,
}

#[derive(Debug, Clone)]
#[sexpr(name = "routes")]
#[pyclass]
pub struct RouteDescriptor {
    pub resolution: pcb::ResolutionDescriptor,
    pub parser: Option<pcb::ParserDescriptor>,
    pub structure_out: Option<StructureOutDescriptor>,
    pub library_out: Option<LibraryOutDescriptor>,
    pub network_out: Option<NetworkOutDescriptor>,
    // TODO: missing fields
}

#[derive(Debug, Clone)]
#[sexpr(name = "structure_out")]
#[pyclass]
pub struct StructureOutDescriptor {
    pub layers: Vec<pcb::structure::LayerDescriptor>,
    pub rule: Option<pcb::structure::RuleDescriptor>,
}

#[derive(Debug, Clone)]
#[sexpr(name = "library_out")]
#[pyclass]
pub struct LibraryOutDescriptor {
    pub padstacks: Vec<pcb::library::PadstackDescriptor>,
    // TODO: missing support for virtual_pin_descriptor
}

#[derive(Debug, Clone)]
#[sexpr(name = "network_out")]
#[pyclass]
pub struct NetworkOutDescriptor {
    pub nets: Vec<NetOutDescriptor>,
}

#[derive(Debug, Clone)]
#[sexpr(name = "net")]
#[pyclass]
pub struct NetOutDescriptor {
    pub net_id: atoms::Id,
    #[sexpr(name = "net_number")]
    pub net_number: Option<numeric::Number>,
    pub rule: Option<pcb::structure::RuleDescriptor>,
    pub wires: Vec<pcb::wiring::WireDescriptor>,
    // TODO: missing fields
}
