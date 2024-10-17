use crate::generators::expressions::gen_expression;
use crate::model::expressions::{Expression, LetInit};
use crate::model::{Ident, Type};
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{ASSIGN_TYPE, COLON_TYPE, COMMA_TYPE, IDENT_TYPE, IN_TYPE, LET_TYPE};
use lexer::model::token::Token;

pub(crate) fn gen_let_expression(
    iter: &mut BufferedTokenIter,
    read_till_token: &Token,
) -> Result<Expression, String> {
    iter.consume_required(&LET_TYPE)?;

    let mut init_list_iter = iter.gen_iter_till(&IN_TYPE);

    let let_init_list = gen_let_init_list(&mut init_list_iter)?;

    iter.consume_required(&IN_TYPE)?;

    // Continue reading till calling code's end-token
    let let_in_expr = gen_expression(iter, read_till_token)?;

    Ok(Expression::Let {
        let_init: let_init_list,
        in_expr: Box::from(let_in_expr),
    })
}

/// ID : TYPE { <- expr } {{, ID : TYPE { <- expr } }}
fn gen_let_init_list(iter: &mut BufferedTokenIter) -> Result<Vec<LetInit>, String> {
    let mut init_list: Vec<LetInit> = Vec::new();

    while iter.has_next() && !iter.peek_eq(&COMMA_TYPE) {
        let init = gen_let_init(iter)?;
        init_list.push(init);

        iter.consume_next_if_eq(&COMMA_TYPE);
    }

    assert!(
        !init_list.is_empty(),
        "Let expression initialisation list is empty"
    );

    Ok(init_list)
}

/// `Id` : `Type` {{ <- expr }}
fn gen_let_init(iter: &mut BufferedTokenIter) -> Result<LetInit, String> {
    let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
        unreachable!()
    };
    let id: Ident = Ident::from(value);

    iter.consume_required(&COLON_TYPE)?;

    let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
        unreachable!()
    };
    let id_type: Type = Type::from(value);

    let mut expr: Option<Expression> = None;

    if iter.peek_eq(&ASSIGN_TYPE) {
        iter.consume_required(&ASSIGN_TYPE)?;

        // read till either encounter ',' or end of iter
        let init_expr = gen_expression(iter, &COMMA_TYPE)?;

        expr = Some(init_expr);
    }

    Ok(LetInit { id, id_type, expr })
}

#[cfg(test)]
mod test {
    use crate::generators::expressions::let_expr::gen_let_expression;
    use crate::test::get_buffered_iter;
    use lexer::model::token::Token;
    use std::fs::File;

    #[test]
    fn test_let_exp() {
        let file = File::open("../test_resources/expressions/expr.let").expect("file not found");
        let mut iter = get_buffered_iter(file);
        let expr =
            gen_let_expression(&mut iter, &Token::EOF).expect("expression generation failure");
        println!("{:?}", expr);
        assert_eq!(expr.get_type(), "Let".to_string());
    }
}
