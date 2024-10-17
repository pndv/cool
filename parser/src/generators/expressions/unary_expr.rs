use crate::generators::expressions;
use crate::model::expressions::Expression;
use lexer::iter::token::BufferedTokenIter;
use lexer::model::constants::{NOT_TYPE, TILDE_TYPE};
use lexer::model::token::Token;

/// {`~` | `not` | `IsVoid`} expr
pub(in crate::generators) fn gen_unary_expr(
    iter: &mut BufferedTokenIter,
    read_till_tokens: &Token,
) -> Result<Expression, String> {
    // match / consume the unary token
    let unary_token = iter
        .next()
        .unwrap_or_else(|| panic!("get_expression_helper: Error reading unary token"));

    let sub_expr = expressions::gen_expression(iter, read_till_tokens)?;

    let unary_expr = if unary_token == NOT_TYPE {
        Expression::Not {
            expr: Box::from(sub_expr),
        }
    } else if unary_token == TILDE_TYPE {
        Expression::Negate {
            expr: Box::from(sub_expr),
        }
    } else {
        Expression::IsVoid {
            expr: Box::from(sub_expr),
        }
    };

    Ok(unary_expr)
}
