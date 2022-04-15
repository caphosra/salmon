use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom_locate::position;

use crate::ast::expr::term::NumberTerm;
use crate::ast::expr::term::ParamTerm;
use crate::ast::expr::term::Term;
use crate::ast::expr::Expression;
use crate::parsers::utils::id_parser;
use crate::parsers::ParserResult;
use crate::parsers::Span;

///
/// Parses a number.
///
pub fn number_parser<'ctx>(input: Span<'ctx>) -> ParserResult<Box<dyn Expression + 'ctx>> {
    let (text, (pos, number)) = tuple((position, digit1))(input)?;

    let number = Box::new(NumberTerm {
        position: pos.into(),
        number: &number,
    });

    Ok((text, number))
}

///
/// Parses a parameter.
///
pub fn param_parser<'ctx>(input: Span<'ctx>) -> ParserResult<Box<dyn Expression + 'ctx>> {
    let (text, (pos, _, name, not_pure)) =
        tuple((position, tag("%"), id_parser, opt(tag("!"))))(input)?;

    let param = Box::new(ParamTerm {
        position: pos.into(),
        name,
        pure: not_pure.is_none(),
    });

    Ok((text, param))
}

///
/// Parses a term.
///
pub fn term_parser<'ctx>(input: Span<'ctx>) -> ParserResult<Box<dyn Expression + 'ctx>> {
    let (text, (pos, values)) = tuple((
        position,
        separated_list1(
            delimited(multispace0, tag("*"), multispace0),
            alt((param_parser, number_parser)),
        ),
    ))(input)?;

    let val = Box::new(Term {
        position: pos.into(),
        values,
    });

    Ok((text, val))
}

#[cfg(test)]
mod test {
    use super::term_parser;
    use crate::assert_expr_eq;
    use crate::ast::expr::term::NumberTerm;
    use crate::ast::expr::term::ParamTerm;
    use crate::ast::expr::term::Term;
    use crate::error::FilePosition;
    use crate::parsers::utils::test::assert_expr_ne;

    #[test]
    fn term_parser_test1() {
        assert_expr_eq!(
            term_parser("123 * %val1 * %val2!"),
            Term {
                position: FilePosition {
                    file_path: "dummy",
                    line: 1,
                    position: 1,
                },
                values: vec![
                    Box::new(NumberTerm {
                        position: FilePosition {
                            file_path: "dummy",
                            line: 1,
                            position: 1,
                        },
                        number: "123",
                    }),
                    Box::new(ParamTerm {
                        position: FilePosition {
                            file_path: "dummy",
                            line: 1,
                            position: 7,
                        },
                        name: "val1".to_string(),
                        pure: true,
                    }),
                    Box::new(ParamTerm {
                        position: FilePosition {
                            file_path: "dummy",
                            line: 1,
                            position: 15,
                        },
                        name: "val2".to_string(),
                        pure: false,
                    }),
                ],
            }
        );
    }

    #[test]
    fn term_parser_test2() {
        assert_expr_ne(term_parser, "^123+456");
    }
}
