use parser_proc_macro::Sexpr;

use crate::{atoms, numeric};

#[derive(Sexpr, Debug)]
#[sexpr(name = "path")]
pub struct PathDescriptor {
    pub layer_id: atoms::Id,
    pub aperature_width: numeric::PositiveDimension,
    pub vertices: Vec<Vertex>,
    pub aperature_type: Option<AperatureType>,
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "rect")]
pub struct RectangleDescriptor {
    pub layer_id: atoms::Id,
    pub corners: (Vertex, Vertex),
}

#[derive(Sexpr, Debug)]
#[sexpr(anonymous)]
pub struct Vertex(pub numeric::Number, pub numeric::Number);

#[derive(Sexpr, Debug, Default)]
pub enum AperatureType {
    #[default]
    Round,
    Square,
}
