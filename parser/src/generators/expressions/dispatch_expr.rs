use crate::generators::expressions::gen_expression;
use crate::model::expressions::Expression;
use crate::model::{Ident, Type};
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{
    AT_TYPE, CLOSE_PAREN_TYPE, COMMA_TYPE, DOT_TYPE, IDENT_TYPE, OPEN_PAREN_TYPE,
};
use lexer::model::token::Token;

pub(super) fn gen_partial_dispatch_expr(
    ident_token: Token,
    iter: &mut BufferedTokenIter,
) -> Result<Expression, String> {
    let Token::Ident { value, .. } = ident_token else {
        unreachable!()
    };
    let fn_name = Ident::from(value);
    let param_list = gen_fn_param_list(iter)?;

    Ok(Expression::PartialDispatch {
        fn_name,
        param_list,
    })
}

/// ...expr (seen before)... { `@` TYPE } `.` ID `(` { expr {{ `,` expr }} }
pub(super) fn gen_partial_cast_dispatch(
    iter: &mut BufferedTokenIter,
) -> Result<Expression, String> {
    let mut cast_type: Option<Type> = None;

    if iter.peek_eq(&AT_TYPE) {
        iter.consume_required(&AT_TYPE)?;

        let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
            unreachable!()
        };
        cast_type = Some(Type::from(value));
    }

    iter.consume_required(&DOT_TYPE)?;

    let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
        unreachable!()
    };
    let fn_name = Ident::from(value);

    let param_list = gen_fn_param_list(iter)?;

    Ok(Expression::PartialCastDispatch {
        cast_type,
        fn_name,
        param_list,
    })
}

fn gen_fn_param_list(iter: &mut BufferedTokenIter) -> Result<Vec<Expression>, String> {
    iter.consume_required(&OPEN_PAREN_TYPE)?;
    let mut fn_param_gen_iter = iter.gen_iter_till(&CLOSE_PAREN_TYPE);
    iter.consume_required(&CLOSE_PAREN_TYPE)?;

    let mut param_list: Vec<Expression> = Vec::new();

    while fn_param_gen_iter.has_next()
        && !fn_param_gen_iter.peek_eq(&COMMA_TYPE)
        && fn_param_gen_iter.has_next()
    {
        let param_expr = gen_expression(&mut fn_param_gen_iter, &COMMA_TYPE)?;
        param_list.push(param_expr);

        // consume ',' if we are not at the end of the stream
        fn_param_gen_iter.consume_next_if_eq(&COMMA_TYPE);
    }

    Ok(param_list)
}
