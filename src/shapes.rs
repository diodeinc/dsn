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

#[derive(Sexpr, Debug, PartialEq)]
#[sexpr(anonymous)]
pub struct Vertex(pub numeric::Number, pub numeric::Number);

#[derive(Sexpr, Debug, Default)]
pub enum AperatureType {
    #[default]
    Round,
    Square,
}

#[derive(Sexpr, Debug)]
#[sexpr(name = "circle")]
pub struct CircleDescriptor {
    pub layer_id: atoms::Id,
    pub diameter: numeric::PositiveDimension,
    pub vertex: Option<Vertex>,
}

#[derive(Sexpr, Debug)]
pub enum ShapeDescriptor {
    #[sexpr(anonymous)]
    Rectangle(RectangleDescriptor),
    #[sexpr(anonymous)]
    Circle(CircleDescriptor),
    // TODO: missing fields
    #[sexpr(anonymous)]
    Path(PathDescriptor),
    // TODO: missing fields
}
