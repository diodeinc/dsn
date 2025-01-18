use chumsky::{error::Simple, prelude::just, text::digits, Parser};
use parser::Parsable;
use parser_proc_macro::Sexpr;
use pyo3::prelude::*;

#[derive(Sexpr, Debug, PartialEq, Clone)]
#[pyclass]
#[sexpr(anonymous)]
pub struct Number {
    pub sign: Option<Sign>,
    pub number_type: NumberType,
}

#[pymethods]
impl Number {
    fn __float__(&self) -> PyResult<f64> {
        Ok(f64::from(Number {
            sign: self.sign.clone(),
            number_type: self.number_type.clone(),
        }))
    }

    fn __int__(&self) -> PyResult<i64> {
        let val = f64::from(Number {
            sign: self.sign.clone(),
            number_type: self.number_type.clone(),
        });
        Ok(val as i64)
    }
}

#[derive(Sexpr, Debug, PartialEq, Clone)]
#[sexpr(anonymous)]
pub enum NumberType {
    #[sexpr(anonymous)]
    Float(Real),
    #[sexpr(anonymous)]
    Rat(Rational),
    #[sexpr(anonymous)]
    PosInt(PositiveInteger),
}

#[derive(Sexpr, Debug, PartialEq, Eq, Clone)]
#[sexpr(anonymous)]
pub enum Sign {
    #[sexpr(name = "+")]
    Plus,
    #[sexpr(name = "-")]
    Minus,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass]
pub struct PositiveInteger(pub u64);

#[pymethods]
impl PositiveInteger {
    fn __int__(&self) -> PyResult<i64> {
        Ok(self.0 as i64)
    }
}

impl PositiveInteger {
    pub fn len(&self) -> u32 {
        self.0.to_string().len() as u32
    }
}

impl From<PositiveInteger> for f64 {
    fn from(value: PositiveInteger) -> Self {
        value.0 as f64
    }
}

impl From<PositiveInteger> for u64 {
    fn from(value: PositiveInteger) -> Self {
        value.0
    }
}

impl Parsable for PositiveInteger {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        digits(10).map(|int: String| Self(int.parse().unwrap()))
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Real(pub f64);

#[pymethods]
impl Real {
    fn __float__(&self) -> PyResult<f64> {
        Ok(self.0)
    }
}

impl Parsable for Real {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        PositiveInteger::parser()
            .then_ignore(just('.'))
            .then(PositiveInteger::parser())
            .map(|(int, frac)| {
                let frac_len = frac.len();
                let int_val: f64 = int.into();
                let frac_val: f64 = frac.into();
                Self(int_val + frac_val / 10_f64.powi(frac_len as i32))
            })
    }
}

impl From<Real> for f64 {
    fn from(value: Real) -> Self {
        value.0
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rational(pub u64, pub u64);

#[pymethods]
impl Rational {
    fn __float__(&self) -> PyResult<f64> {
        Ok(f64::from(self.clone()))
    }

    #[getter]
    fn numerator(&self) -> u64 {
        self.0
    }

    #[getter]
    fn denominator(&self) -> u64 {
        self.1
    }
}

impl Parsable for Rational {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        PositiveInteger::parser()
            .then_ignore(just('/'))
            .then(PositiveInteger::parser())
            .map(|(num, denom)| Self(num.into(), denom.into()))
    }
}

impl From<Rational> for f64 {
    fn from(value: Rational) -> Self {
        value.0 as f64 / value.1 as f64
    }
}

#[derive(Sexpr, Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum DimensionUnit {
    Inch,
    Mil,
    Cm,
    Mm,
    Um,
}

#[derive(Sexpr, Debug, Clone)]
#[sexpr(anonymous)]
#[pyclass]
// TODO: Technically this is broader than it should be, we should reject a negative number here.
pub struct PositiveDimension(pub Number);

#[pymethods]
impl PositiveDimension {
    fn __float__(&self) -> PyResult<f64> {
        Ok(f64::from(self.0.clone()))
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        let mult: f64 = match value.sign {
            Some(Sign::Minus) => -1.0,
            _ => 1.0,
        };
        match value.number_type {
            NumberType::PosInt(int) => {
                let v: f64 = int.into();
                v * mult
            }
            NumberType::Float(real) => {
                let v: f64 = real.into();
                v * mult
            }
            NumberType::Rat(rational) => {
                let v: f64 = rational.into();
                v * mult
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        assert_eq!(
            PositiveInteger::parser().parse("123").unwrap(),
            PositiveInteger(123)
        );
    }

    #[test]
    fn test_real() {
        assert_eq!(Real::parser().parse("123.456").unwrap(), Real(123.456));
    }

    #[test]
    fn test_rational() {
        assert_eq!(
            Rational::parser().parse("123/456").unwrap(),
            Rational(123, 456)
        );
    }

    #[test]
    fn test_number() {
        let f: f64 = Number::parser().parse("123").unwrap().into();
        assert_eq!(f, 123.0);

        let f: f64 = Number::parser().parse("123.456").unwrap().into();
        assert_eq!(f, 123.456);

        let f: f64 = Number::parser().parse("123/456").unwrap().into();
        assert_eq!(f, 123.0 / 456.0);

        let f: f64 = Number::parser().parse("-123.456").unwrap().into();
        assert_eq!(f, -123.456);

        let f: f64 = Number::parser().parse("-123/456").unwrap().into();
        assert_eq!(f, -123.0 / 456.0);
    }
}
