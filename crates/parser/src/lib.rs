use ariadne::{Label, Report, ReportKind, Source};
use chumsky::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Token {
    String(String),
    Keyword(String),
    Number(String),
    LParen,
    RParen,
}

pub trait Parsable<'a> {
    fn parser() -> BoxedParser<'a, char, Self, Simple<char>>
    where
        Self: Sized;
}

impl<'a> Parsable<'a> for () {
    fn parser() -> BoxedParser<'a, char, (), Simple<char>> {
        empty().padded().boxed()
    }
}

impl<'a, T> Parsable<'a> for Option<T>
where
    T: Parsable<'a> + 'a,
{
    fn parser() -> BoxedParser<'a, char, Self, Simple<char>> {
        T::parser().or_not().padded().boxed()
    }
}

impl<'a, T> Parsable<'a> for (T, T)
where
    T: Parsable<'a> + 'a,
{
    fn parser() -> BoxedParser<'a, char, Self, Simple<char>> {
        T::parser().then(T::parser()).padded().boxed()
    }
}

impl<'a, T> Parsable<'a> for Vec<T>
where
    T: Parsable<'a> + 'a,
{
    fn parser() -> BoxedParser<'a, char, Self, Simple<char>> {
        T::parser().repeated().collect().padded().boxed()
    }
}

pub fn lparen() -> impl Parser<char, (), Error = Simple<char>> {
    just('(').map(|_| ())
}

pub fn rparen() -> impl Parser<char, (), Error = Simple<char>> {
    just(')').map(|_| ())
}

pub fn keyword<'a>(keyword: &'a str) -> impl Parser<char, (), Error = Simple<char>> + 'a {
    just(keyword).map(|_| ()).padded()
}

pub struct FieldConfig<'a, T> {
    pub name: &'a str,
    pub parser: BoxedParser<'a, char, T, Simple<char>>,
    pub anonymous: bool,
}

impl<'a, T> FieldConfig<'a, T> {
    pub fn new(name: &'a str, anonymous: bool) -> FieldConfig<'a, T>
    where
        T: Parsable<'a> + 'a,
    {
        FieldConfig {
            name,
            parser: T::parser().boxed(),
            anonymous,
        }
    }
}

pub fn field<'a, T: 'a>(
    config: FieldConfig<'a, T>,
) -> impl Parser<char, T, Error = Simple<char>> + 'a {
    if config.anonymous {
        config.parser.padded().boxed()
    } else {
        lparen()
            .ignore_then(keyword(config.name))
            .ignore_then(config.parser.padded())
            .then_ignore(rparen())
            .padded()
            .boxed()
    }
}

impl<'a> Parsable<'a> for String {
    fn parser() -> BoxedParser<'a, char, Self, Simple<char>> {
        let quoted = just('"')
            .ignore_then(
                filter(|c: &char| *c != '"' && *c != '\n')
                    .repeated()
                    .map(|chars: Vec<char>| chars.into_iter().collect()),
            )
            .then_ignore(just('"'))
            .padded();

        let unquoted = filter(|c: &char| {
            !c.is_whitespace() && *c != '(' && *c != ')' && *c != '"' && *c != '\n'
        })
        .repeated()
        .at_least(1)
        .map(|chars: Vec<char>| chars.into_iter().collect())
        .padded();

        quoted.or(unquoted).boxed()
    }
}

pub trait PrettyPrintError {
    fn pretty_print(&self, input: &str);
}

impl PrettyPrintError for &Simple<char> {
    fn pretty_print(&self, input: &str) {
        let report = Report::build(ReportKind::Error, self.span())
            .with_code(3)
            .with_label(Label::new(self.span()).with_message(self.to_string()))
            .finish();

        report
            .print(Source::from(input))
            .expect("failed to print error");
    }
}
