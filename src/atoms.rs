use std::ops::Deref;

use chumsky::prelude::*;
use parser::Parsable;
use parser_proc_macro::Sexpr;

#[derive(Sexpr, Debug, PartialEq, Eq)]
#[sexpr(anonymous)]
pub struct Id(String);

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

#[derive(Debug)]
pub struct Bool(bool);

impl Parsable for Bool {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        choice((
            just("on").map(|_| Self(true)),
            just("off").map(|_| Self(false)),
        ))
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
