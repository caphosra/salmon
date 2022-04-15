use nom::error::ParseError;

use crate::parsers::Span;

///
/// To resolve the conflict around `ErrorKind`.
///
type NomErrorKind = nom::error::ErrorKind;

///
/// Holds a type of the error.
///
/// `Syntax` prefix means that it's a kind of a syntax error,
/// while `Analysis` prefix does an error on analyzing the code.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    SyntaxNom(NomErrorKind),
    SyntaxChar(char),
}

///
/// Contains where the error ocurred.
///
#[derive(Clone, Debug, PartialEq)]
pub struct FilePosition<'ctx> {
    pub file_path: &'ctx str,
    pub line: u32,
    pub position: u32,
}

///
/// Converts `LocatedSpan` into `FilePosition`.
///
impl<'ctx> From<Span<'ctx>> for FilePosition<'ctx> {
    fn from(input: Span<'ctx>) -> Self {
        let file_path = input.extra;
        let line = input.location_line();
        let position = input.get_column() as u32;
        Self {
            file_path,
            line,
            position,
        }
    }
}

///
/// Represents a list of errors which are ocurred while processing sources.
///
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralError<'ctx> {
    pub errors: Vec<(FilePosition<'ctx>, ErrorKind)>,
}

///
/// To use a `GeneralError` as a `ParserError`.
///
impl<'ctx> ParseError<Span<'ctx>> for GeneralError<'ctx> {
    fn from_error_kind(input: Span<'ctx>, kind: NomErrorKind) -> Self {
        Self {
            errors: vec![(input.into(), ErrorKind::SyntaxNom(kind))],
        }
    }

    fn append(input: Span<'ctx>, kind: NomErrorKind, other: Self) -> Self {
        let mut other = other.clone();
        other
            .errors
            .push((input.into(), ErrorKind::SyntaxNom(kind)));
        other
    }

    fn from_char(input: Span<'ctx>, c: char) -> Self {
        Self {
            errors: vec![(input.into(), ErrorKind::SyntaxChar(c))],
        }
    }

    fn or(self, other: Self) -> Self {
        other
    }
}

impl<'ctx> GeneralError<'ctx> {
    pub fn new(position: FilePosition<'ctx>, error: ErrorKind) -> Self {
        Self {
            errors: vec![(position, error)],
        }
    }
}
