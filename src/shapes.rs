use crate::{atoms, numeric};
use chumsky::Parser;
use parser::Parsable;
use parser_proc_macro::Sexpr;
use pyo3::prelude::*;

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "path")]
#[pyclass]
pub struct PathDescriptor {
    #[pyo3(get, set)]
    pub layer_id: atoms::Id,
    #[pyo3(get, set)]
    pub aperature_width: numeric::PositiveDimension,
    #[pyo3(get, set)]
    pub vertices: Vec<Vertex>,
    #[pyo3(get, set)]
    pub aperature_type: Option<AperatureType>,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "rect")]
#[pyclass]
pub struct RectangleDescriptor {
    #[pyo3(get, set)]
    pub layer_id: atoms::Id,
    #[pyo3(get, set)]
    pub corners: (Vertex, Vertex),
}

#[derive(Sexpr, Debug, PartialEq, Clone)]
#[sexpr(anonymous)]
#[pyclass]
pub struct Vertex(
    #[pyo3(get, set, name = "x")] pub numeric::Number,
    #[pyo3(get, set, name = "y")] pub numeric::Number,
);

#[derive(Sexpr, Debug, Default, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum AperatureType {
    #[default]
    Round,
    Square,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "circle")]
#[pyclass]
pub struct CircleDescriptor {
    #[pyo3(get, set)]
    pub layer_id: atoms::Id,
    #[pyo3(get, set)]
    pub diameter: numeric::PositiveDimension,
    #[pyo3(get, set)]
    pub vertex: Option<Vertex>,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(name = "polygon")]
#[pyclass]
pub struct PolygonDescriptor {
    #[pyo3(get, set)]
    pub layer_id: atoms::Id,
    #[pyo3(get, set)]
    pub aperature_width: numeric::PositiveDimension,
    #[pyo3(get, set)]
    pub vertices: Vec<Vertex>,
    #[pyo3(get, set)]
    pub aperature_type: Option<AperatureType>,
}

#[derive(Sexpr, Debug, Clone)]
#[pyclass]
pub enum ShapeDescriptor {
    #[sexpr(anonymous)]
    Rectangle(RectangleDescriptor),
    #[sexpr(anonymous)]
    Circle(CircleDescriptor),
    #[sexpr(anonymous)]
    Polygon(PolygonDescriptor),
    // TODO: missing fields
    #[sexpr(anonymous)]
    Path(PathDescriptor),
    // TODO: missing fields
}
