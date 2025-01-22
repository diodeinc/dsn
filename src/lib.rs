use pyo3::prelude::*;

pub mod atoms;
pub mod numeric;
pub mod pcb;
pub mod shapes;

#[pymodule]
fn dsn(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let numeric_module = PyModule::new(py, "numeric")?;

    numeric_module.add_class::<numeric::Number>()?;
    numeric_module.add_class::<numeric::Real>()?;
    numeric_module.add_class::<numeric::Rational>()?;

    let shapes_module = PyModule::new(py, "shapes")?;

    shapes_module.add_class::<shapes::PathDescriptor>()?;
    shapes_module.add_class::<shapes::RectangleDescriptor>()?;
    shapes_module.add_class::<shapes::Vertex>()?;
    shapes_module.add_class::<shapes::AperatureType>()?;
    shapes_module.add_class::<shapes::CircleDescriptor>()?;
    shapes_module.add_class::<shapes::ShapeDescriptor>()?;

    m.add_submodule(&numeric_module)?;
    m.add_submodule(&shapes_module)?;

    // Add main pcb types
    m.add_class::<pcb::Pcb>()?;
    m.add_class::<pcb::ParserDescriptor>()?;
    m.add_class::<pcb::ResolutionDescriptor>()?;
    m.add_class::<pcb::UnitDescriptor>()?;
    m.add_class::<pcb::Constant>()?;
    m.add_class::<pcb::QuoteChar>()?;

    // Add structure types
    m.add_class::<pcb::structure::StructureDescriptor>()?;
    m.add_class::<pcb::structure::LayerDescriptor>()?;
    m.add_class::<pcb::structure::LayerType>()?;
    m.add_class::<pcb::structure::UserPropertyDescriptor>()?;
    m.add_class::<pcb::structure::PropertyValueDescriptor>()?;
    m.add_class::<pcb::structure::PropertyValue>()?;
    m.add_class::<pcb::structure::DirectionType>()?;
    m.add_class::<pcb::structure::BoundaryDescriptor>()?;
    m.add_class::<pcb::structure::BoundaryDescriptorType>()?;
    m.add_class::<pcb::structure::RuleDescriptor>()?;
    m.add_class::<pcb::structure::RuleDescriptorType>()?;
    m.add_class::<pcb::structure::ClearanceDescriptor>()?;
    m.add_class::<pcb::structure::ClearanceType>()?;
    m.add_class::<pcb::structure::ViaDescriptor>()?;
    m.add_class::<pcb::structure::PadstackId>()?;

    // Add placement types
    m.add_class::<pcb::placement::PlacementDescriptor>()?;
    m.add_class::<pcb::placement::ComponentInstance>()?;
    m.add_class::<pcb::placement::PlacementReference>()?;
    m.add_class::<pcb::placement::PlacementReferenceLocation>()?;
    m.add_class::<pcb::placement::Side>()?;

    // Add library types
    m.add_class::<pcb::library::LibraryDescriptor>()?;
    m.add_class::<pcb::library::ImageDescriptor>()?;
    m.add_class::<pcb::library::Side>()?;
    m.add_class::<pcb::library::OutlineDescriptor>()?;
    m.add_class::<pcb::library::PinDescriptor>()?;
    m.add_class::<pcb::library::PinRefDescriptor>()?;
    m.add_class::<pcb::library::JumperDescriptor>()?;
    m.add_class::<pcb::library::PadstackDescriptor>()?;
    m.add_class::<pcb::library::PadstackShapeDescriptor>()?;
    m.add_class::<pcb::library::WindowDescriptor>()?;
    m.add_class::<pcb::library::AttachDescriptor>()?;
    m.add_class::<pcb::library::PadViaSiteDescriptor>()?;
    m.add_class::<pcb::library::KeepoutDescriptor>()?;

    // Add network types
    m.add_class::<pcb::network::NetworkDescriptor>()?;
    m.add_class::<pcb::network::NetDescriptor>()?;
    m.add_class::<pcb::network::PinReference>()?;
    m.add_class::<pcb::network::NetPinsOrOrder>()?;
    m.add_class::<pcb::network::ClassDescriptor>()?;
    m.add_class::<pcb::network::NetOrCompositeList>()?;
    m.add_class::<pcb::network::CompositeNameList>()?;
    m.add_class::<pcb::network::CircuitDescriptor>()?;
    m.add_class::<pcb::network::LayerRuleDescriptor>()?;
    m.add_class::<pcb::network::CircuitDescriptorType>()?;

    // Add wiring types
    m.add_class::<pcb::wiring::WiringDescriptor>()?;
    m.add_class::<pcb::wiring::WireDescriptor>()?;
    m.add_class::<pcb::wiring::WireShapeDescriptor>()?;
    m.add_class::<pcb::wiring::WireType>()?;
    m.add_class::<pcb::wiring::WireViaDescriptor>()?;

    Ok(())
}
