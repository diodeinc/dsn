use pyo3::prelude::*;

pub mod atoms;
pub mod numeric;
pub mod pcb;
pub mod shapes;

#[pymodule]
fn dsn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<numeric::Number>()?;
    m.add_class::<numeric::Real>()?;
    m.add_class::<numeric::Rational>()?;
    Ok(())
}
