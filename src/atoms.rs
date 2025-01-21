use std::ops::Deref;

use chumsky::prelude::*;
use parser::Parsable;
use parser_proc_macro::Sexpr;
use pyo3::prelude::*;

#[derive(Sexpr, Debug, PartialEq, Eq, Clone)]
#[pyclass]
#[sexpr(anonymous)]
pub struct Id(String);

#[pymethods]
impl Id {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.clone())
    }
}

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Bool(bool);

#[pymethods]
impl Bool {
    fn __bool__(&self) -> PyResult<bool> {
        Ok(self.0)
    }
}

impl<'a> Parsable<'a> for Bool {
    fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
        choice((
            just("on").map(|_| Self(true)),
            just("off").map(|_| Self(false)),
        ))
        .boxed()
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
