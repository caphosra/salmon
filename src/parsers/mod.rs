use nom::IResult;
use nom_locate::LocatedSpan;

use crate::error::GeneralError;

///
/// An input which also contains meta information.
///
/// The `extra` field is used to hold a path to the source file.
///
pub type Span<'ctx> = LocatedSpan<&'ctx str, &'ctx str>;

///
/// Holds the result of parsing the code.
///
pub type ParserResult<'ctx, T> = IResult<Span<'ctx>, T, GeneralError<'ctx>>;

pub mod expr;
pub mod utils;
