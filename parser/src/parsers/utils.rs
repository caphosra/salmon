use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::sequence::tuple;

use crate::parsers::ParserResult;
use crate::parsers::Span;

///
/// Recognizes a named value or function.
///
pub fn id_parser(input: Span) -> ParserResult<String> {
    let (text, (first, val)) = tuple((alpha1, alphanumeric0))(input)?;

    let id = first.to_string() + &val;

    Ok((text, id))
}

#[cfg(test)]
pub mod test {
    use std::fmt::Debug;

    use crate::ast::expr::Expression;
    use crate::parsers::utils::ParserResult;
    use crate::parsers::Span;

    #[macro_export]
    macro_rules! assert_expr_eq {
        ($parser:ident($input:expr), $should_be:expr) => {{
            let (_, value) = $parser($crate::parsers::Span::new_extra($input, "dummy")).unwrap();
            assert_eq!(format!("{:?}", value), format!("{:?}", $should_be));
        }};
    }

    pub fn assert_expr_ne<'ctx, F>(parser: F, expr: &'ctx str)
    where
        F: Fn(Span<'ctx>) -> ParserResult<Box<dyn Expression + 'ctx>>,
    {
        assert!(parser(Span::new_extra(expr, "dummy")).is_err());
    }

    #[macro_export]
    macro_rules! init_expr {
        ($expr_ty:ident { pos : ($line:expr, $position:expr), $($prop:ident : $val:expr,)*}) => {{
            Box::new($expr_ty {
                position: $crate::error::FilePosition {
                    file_path: "dummy",
                    line: $line,
                    position: $position,
                },
                $($prop : $val,)*
            })
        }};
    }
}
