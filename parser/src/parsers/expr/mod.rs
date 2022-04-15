use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom_locate::position;

use crate::ast::expr::BinaryOp;
use crate::ast::expr::BinaryOpExpr;
use crate::ast::expr::Expression;
use crate::parsers::expr::term::term_parser;
use crate::parsers::ParserResult;
use crate::parsers::Span;

pub fn expr_parser<'ctx>(input: Span<'ctx>) -> ParserResult<Box<dyn Expression + 'ctx>> {
    let (text, (pos, left, right)) = tuple((
        position,
        term_parser,
        opt(tuple((
            delimited(multispace0, alt((tag("+"), tag("-"))), multispace0),
            expr_parser,
        ))),
    ))(input)?;

    if let Some((op, right)) = right {
        let op = match op.fragment() {
            &"+" => BinaryOp::Add,
            &"-" => BinaryOp::Sub,
            _ => panic!("The operator must be \"+\" or \"-\"."),
        };

        Ok((
            text,
            Box::new(BinaryOpExpr {
                position: pos.into(),
                op,
                left,
                right,
            }),
        ))
    } else {
        Ok((text, left))
    }
}

pub mod term;

#[cfg(test)]
mod test {
    use super::expr_parser;
    use crate::assert_expr_eq;
    use crate::ast::expr::term::NumberTerm;
    use crate::ast::expr::term::ParamTerm;
    use crate::ast::expr::term::Term;
    use crate::ast::expr::BinaryOp;
    use crate::ast::expr::BinaryOpExpr;
    use crate::init_expr;

    #[test]
    fn term_parser_test1() {
        assert_expr_eq!(
            expr_parser("123 + 314 * %val"),
            init_expr!(BinaryOpExpr {
                pos: (1, 1),
                op: BinaryOp::Add,
                left: init_expr!(Term {
                    pos: (1, 1),
                    values: vec![init_expr!(NumberTerm {
                        pos: (1, 1),
                        number: "123",
                    })],
                }),
                right: init_expr!(Term {
                    pos: (1, 7),
                    values: vec![
                        init_expr!(NumberTerm {
                            pos: (1, 7),
                            number: "314",
                        }),
                        init_expr!(ParamTerm {
                            pos: (1, 13),
                            name: "val".to_string(),
                            pure: true,
                        }),
                    ],
                }),
            })
        );
    }
}
