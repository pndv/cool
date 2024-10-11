use crate::nodes::{Expression, Formal, Id, LetInit, Program, Type};
use crate::tokens::{consume_required, Token, ASSIGN_TYPE, CASE_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, COLON_TYPE, COMMA_TYPE, DOT_TYPE, ELSE_TYPE, END_CASE_TYPE, END_IF_TYPE, END_LOOP_TYPE, IDENT_TYPE, IF_TYPE, IN_TYPE, LAMBDA_TYPE, LET_TYPE, LOOP_TYPE, OF_TYPE, SEMI_COLON_TYPE, THEN_TYPE, WHILE_TYPE};
use core::iter::Iterator;
use expressions::case_expr::CaseBranch;
use tokens::{match_required_token, FilteredTokensIterator};
use crate::class::Class;

pub mod scanner;
pub mod nodes;
pub mod tokens;
mod expressions;
mod feature;
mod class;
mod formal;
mod token_iter;

#[must_use]
pub fn parse_program_from_file(file_path: &str) -> Program {
  let mut token_iter: FilteredTokensIterator = tokens::get_filtered_token_iter(file_path);

  get_program(&mut token_iter)
}

/// Program is a list of semicolon separated classes 
fn get_program(token_iter: &mut FilteredTokensIterator) -> Program {
  let mut program: Program = Program::new();

  while token_iter.peek().is_some() {
    let class: Class = class::gen_class(token_iter, &SEMI_COLON_TYPE);
    match_required_token(token_iter.next(), SEMI_COLON_TYPE);
    program.add_class(class);
  }

  program
}

/// Formals |-> formal {{, formals}}
fn get_formals(token_iter: &mut FilteredTokensIterator) -> Vec<Formal> {
  let mut formals: Vec<Formal> = Vec::new();
  let mut formal = get_formal(token_iter);
  formals.push(formal);

  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&COMMA_TYPE) {
    match_required_token(token_iter.next(), COMMA_TYPE); // Consume ','

    formal = get_formal(token_iter);
    formals.push(formal);
  }

  formals
}

/// Formal |-> ID : TYPE
fn get_formal(token_iter: &mut FilteredTokensIterator) -> Formal {
  let mut token = match_required_token(token_iter.next(), IDENT_TYPE);
  let formal_name: Id = Id::from(token);

  match_required_token(token_iter.next(), COLON_TYPE); // consume colon

  token = match_required_token(token_iter.next(), IDENT_TYPE);
  let formal_type: Type = Type::from(token);

  (formal_name, formal_type).into()
}

fn get_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  assert!(token_iter.peek().is_some(), "Unexpected EOF"); // assert that there are some tokens remaining

  let mut stack: Vec<Expression> = Vec::new();
  fill_expression_stack(token_iter, &mut stack);

  // The entire stack must reduce to a single `Expression`
  if stack.is_empty() { return Expression::NoExpr; }

  let mut expr = Expression::NoExpr;

  if stack.len() == 1 {
    expr = stack[0].clone();

    // If first expression is a PartialDispatch convert it to Dispatch by adding `self`
    if let Expression::PartialDispatch { fn_name, param_list } = expr {
      expr = Expression::Dispatch {
        calling_expr: Box::from(Expression::SelfExpr),
        cast_type: None,
        fn_name,
        param_list,
      }
    }

    return expr;
  }

  for i in 1..stack.len() {
    let next_expr = stack[i].clone();
    assert!(next_expr.is_partial());

    match next_expr {
      Expression::PartialBinary { binary_token, right_expr } => {
        match binary_token {
          Token::Star { .. } => expr = Expression::Multiply { left: Box::from(stack[0].clone()), right: right_expr },
          Token::ForwardSlash { .. } => expr = Expression::Divide { left: Box::from(stack[0].clone()), right: right_expr },
          Token::Plus { .. } => expr = Expression::Plus { left: Box::from(stack[0].clone()), right: right_expr },
          Token::Minus { .. } => expr = Expression::Minus { left: Box::from(stack[0].clone()), right: right_expr },
          Token::LessOrEqual { .. } => expr = Expression::LessThanOrEqual { left: Box::from(stack[0].clone()), right: right_expr },
          Token::Less { .. } => expr = Expression::LessThan { left: Box::from(stack[0].clone()), right: right_expr },
          Token::Equal { .. } => expr = Expression::Equal { left: Box::from(stack[0].clone()), right: right_expr },
          _ => panic!("Not a binary token {:?}", binary_token),
        }
      }

      Expression::PartialDispatch { fn_name, param_list } => {
        let first = stack[0].clone();
        expr = Expression::Dispatch { calling_expr: Box::from(first), cast_type: None, fn_name, param_list }
      }
      Expression::PartialCastDispatch { cast_type, fn_name, param_list } => {
        let first = stack[0].clone();
        expr = Expression::Dispatch { calling_expr: Box::from(first), cast_type, fn_name, param_list };
      }

      _ => panic!("Should be partial expression, received {:?}", next_expr.get_type())
    }
  }

  expr
}

fn fill_expression_stack(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) {
  let Some(token) = token_iter.peek() else { panic!("Unexpected EOF") };
  let expr: Expression;
  match token {
    Token::Ident { .. } => expr = get_expr_from_ident(token_iter, stack),

    Token::If { .. } => expr = get_conditional_expr(token_iter, stack),

    Token::While { .. } => expr = get_loop_expr(token_iter, stack),

    Token::OpenCurl { .. } => expr = get_block_expr(token_iter, stack),

    Token::Let { .. } => expr = get_let_expr(token_iter, stack),

    Token::Case { .. } => expr = get_case_expr(token_iter, stack),

    Token::At { .. } => { // Dynamic dispatch
      let _ = token_iter.next(); // consume `@`
      let ident_type = match_required_token(token_iter.next(), IDENT_TYPE); // consume `TYPE`
      let cast_type = Some(Type::from(ident_type));

      // `.` must follow the cast_type
      assert!(token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&DOT_TYPE));

      let dispatch_expr = get_stack_top(token_iter, stack);
      if let Expression::PartialDispatch { fn_name, param_list } = dispatch_expr {
        expr = Expression::PartialCastDispatch { cast_type, fn_name, param_list };
      } else {
        dbg!(&dispatch_expr);
        panic!("Invalid state for dynamic dispatch expression. Expected PartialDispatch type found {:?}", dispatch_expr.get_type());
      }
    }

    Token::Dot { .. } => {
      match_required_token(token_iter.next(), DOT_TYPE); // consume `.`

      // `Ident` must follow the `.`
      assert!(token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&IDENT_TYPE));

      let dispatch_expr = get_stack_top(token_iter, stack);
      if let Expression::PartialDispatch { .. } = dispatch_expr {
        expr = dispatch_expr;
      } else {
        panic!("Invalid state for dynamic dispatch expression. Expected PartialDispatch type found {:?}", dispatch_expr.get_type());
      }
    }

    Token::New { .. } => {
      let _ = token_iter.next(); // consume `new`
      let Some(type_token) = token_iter.next() else { panic!("Unexpected EOF") }; // consume `type` i.e. Ident
      let type_name: Type = Type::from(type_token);

      expr = Expression::New { type_name };
    }

    Token::IsVoid { .. } => {
      let _ = token_iter.next(); // consume `IsVoid`
      let void_expr = get_stack_top(token_iter, stack);
      expr = Expression::IsVoid { expr: Box::from(void_expr) };
    }

    Token::Assign { .. } => {
      match_required_token(token_iter.next(), ASSIGN_TYPE);
      let assign_expr = get_expression(token_iter);
      expr = Expression::PartialAssign { expr: Box::from(assign_expr) };
    }

    Token::Plus { .. } |
    Token::Minus { .. } |
    Token::Star { .. } |
    Token::ForwardSlash { .. } |
    Token::Less { .. } |
    Token::LessOrEqual { .. } |
    Token::Equal { .. } => {     //binary expression
      let binary_token = token_iter.next().unwrap();
      let right_expr = get_stack_top(token_iter, stack);
      expr = Expression::PartialBinary { binary_token, right_expr: Box::from(right_expr) };
    }

    Token::Tilde { .. } => {
      let _ = token_iter.next(); // consume `~`
      let tilde_expr = get_stack_top(token_iter, stack);
      expr = Expression::Negate { expr: Box::from(tilde_expr) };
    }

    Token::Not { .. } => {
      let _ = token_iter.next(); // consume `Not`
      let not_expr = get_stack_top(token_iter, stack);
      expr = Expression::Not { expr: Box::from(not_expr) };
    }

    Token::OpenParen { .. } => {
      let _ = token_iter.next(); // consume `(`
      expr = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);
    }

    Token::Int { .. } | Token::String { .. } | Token::True { .. } | Token::False { .. } => {
      let t: Token = token_iter.next().unwrap();
      expr = Expression::from(t);
    }

    // Intermediate expression, return as-is, handled by their respective branches 
    Token::SemiColon { .. } | Token::Comma { .. } |
    Token::Then { .. } | Token::Else { .. } | Token::EndIf { .. } |
    Token::Loop { .. } | Token::EndLoop { .. } |
    Token::In { .. } |
    Token::Of { .. } | Token::EndCase { .. } |
    Token::CloseParen { .. } | Token::CloseCurl { .. } => return,

    _ => expr = Expression::NoExpr,
  }
  stack.push(expr);
}

fn get_case_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  consume_required(token_iter, CASE_TYPE); // consume `case`

  let switch_expression = get_stack_top(token_iter, stack);

  consume_required(token_iter, OF_TYPE); // consume `of`

  let branches = get_case_branch_list(token_iter, stack);

  consume_required(token_iter, END_CASE_TYPE); // consume `esac` 

  Expression::Case { switch_expression: Box::from(switch_expression), branches }
}

fn get_let_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  match_required_token(token_iter.next(), LET_TYPE); // consume `let`
  let expr_list = get_let_init_list(token_iter, stack);

  match_required_token(token_iter.next(), IN_TYPE); // consume `in`
  let in_expr = get_stack_top(token_iter, stack);

  Expression::Let { let_init: expr_list, in_expr: Box::from(in_expr) }
}

fn get_block_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  let _ = token_iter.next(); // consume `{`

  let expressions = get_block_expr_list(token_iter, stack);

  match_required_token(token_iter.next(), CLOSE_CURL_TYPE); // consume `}`

  Expression::Block { expr_list: expressions }
}

fn get_loop_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  match_required_token(token_iter.next(), WHILE_TYPE); // consume `while`

  let predicate = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), LOOP_TYPE);

  let body = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), END_LOOP_TYPE);

  Expression::Loop { predicate: Box::from(predicate), body: Box::from(body) }
}

fn get_conditional_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  match_required_token(token_iter.next(), IF_TYPE); // consume `If`
  let predicate = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), THEN_TYPE); // consume `Then`
  let then_expr = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), ELSE_TYPE); // consume `Else`
  let else_expr = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), END_IF_TYPE); // consume `Fi`

  Expression::Conditional {
    predicate: Box::from(predicate),
    then_expr: Box::from(then_expr),
    else_expr: Box::from(else_expr),
  }
}

/// The expression from ID could be:
/// 1. ID <- Type
/// 2. ID ( {{expr}}+ ) -- dispatch (static or dynamic)
/// 3. ID
fn get_expr_from_ident(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  let ident = match_required_token(token_iter.next(), IDENT_TYPE); // consume the identifier
  let expr: Expression;

  let peeked_token = token_iter.peek();

  if let Some(next_token) = peeked_token {
    match next_token {
      Token::Assign { .. } => {
        let _ = token_iter.next(); // consume `<-`
        let symbol: Id = Id::from(ident);
        let assign_expr = get_stack_top(token_iter, stack);
        expr = Expression::Assign { name: symbol, expr: Box::from(assign_expr) }
      }
      Token::OpenParen { .. } => {
        let _ = token_iter.next(); // consume `(`

        let function_name: Id = Id::from(ident);
        let param_list: Vec<Expression> = get_fn_param_expr_list(token_iter, stack);

        match_required_token(token_iter.next(), CLOSE_PAREN_TYPE); // consume `)`

        // self.`fn(params`
        expr = Expression::PartialDispatch {
          fn_name: function_name,
          param_list,
        }
      }
      _ => expr = Expression::from(ident),
    }
  } else {
    // Nothing after Identifier, return plain Ident expression
    expr = Expression::from(ident);
  }

  expr
}

fn get_stack_top(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  let count = stack.len();
  fill_expression_stack(token_iter, stack);
  assert_eq!(stack.len(), count + 1); // should have only one more expression on the stack
  stack.pop().unwrap()
}

fn get_block_expr_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Vec<Expression> {
  let mut block_expr_list: Vec<Expression> = Vec::new();

  let expr = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), SEMI_COLON_TYPE);
  block_expr_list.push(expr);

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&CLOSE_CURL_TYPE) {
    let expr = get_stack_top(token_iter, stack);
    match_required_token(token_iter.next(), SEMI_COLON_TYPE);
    block_expr_list.push(expr);
  }

  block_expr_list
}

fn get_let_init_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Vec<LetInit> {
  /// matches `Id:Type [[ <- Expression ]]`
  fn get_let_init(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> LetInit {
    let ident_token = match_required_token(token_iter.next(), IDENT_TYPE);
    let ident_name: Id = Id::from(ident_token);

    match_required_token(token_iter.next(), COLON_TYPE);

    let ident_type_token = match_required_token(token_iter.next(), IDENT_TYPE);
    let ident_type: Type = Type::from(ident_type_token);

    let mut expr: Option<Box<Expression>> = None;
    let peek = token_iter.peek();
    if peek.is_some() && peek.unwrap().is_same_type(&ASSIGN_TYPE) {
      match_required_token(token_iter.next(), ASSIGN_TYPE); // consume `<-`
      let expression = get_stack_top(token_iter, stack);
      expr = Some(Box::from(expression));
    }

    (ident_name, ident_type, expr)
  }

  let mut list: Vec<LetInit> = Vec::new();

  let first_init = get_let_init(token_iter, stack);
  list.push(first_init);

  // get `LetInit` while there is `Comma`
  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&COMMA_TYPE) {
    let init = get_let_init(token_iter, stack);
    list.push(init);
  }

  list
}

/// Get list of all branches of `case` statement
fn get_case_branch_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Vec<CaseBranch> {
  /// matches `Id:Type => Expression;`
  fn get_case_branch(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> CaseBranch {
    let ident_token = match_required_token(token_iter.next(), IDENT_TYPE);
    let ident_name: Id = Id::from(ident_token);

    match_required_token(token_iter.next(), COLON_TYPE);

    let type_token = match_required_token(token_iter.next(), IDENT_TYPE);
    let ident_type: Type = Type::from(type_token);

    match_required_token(token_iter.next(), LAMBDA_TYPE);
    let expr = get_stack_top(token_iter, stack);

    match_required_token(token_iter.next(), SEMI_COLON_TYPE);

    (ident_name, ident_type, Box::from(expr))
  }

  let mut case_branch_list: Vec<CaseBranch> = Vec::new();

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&END_CASE_TYPE) {
    case_branch_list.push(get_case_branch(token_iter, stack));
  }

  case_branch_list
}

/// Get list of parameters for function call (dynamic and static dispatch)
fn get_fn_param_expr_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Vec<Expression> {
  let mut expr_list: Vec<Expression> = Vec::new();

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&CLOSE_PAREN_TYPE) {
    let expr = get_stack_top(token_iter, stack);
    expr_list.push(expr);

    if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&COMMA_TYPE) {
      // consume `,` continue with the loop
      match_required_token(token_iter.next(), COMMA_TYPE);
    }
  }

  expr_list
}

#[cfg(test)]
mod test {
  use crate::parse_program_from_file;

  #[test]
  fn test_single_program() {
    let file = "test_resources/cool.cl";
    let program = parse_program_from_file(file);
    println!("{:#?}", program);
  }

  #[test]
  fn test_large_program() {
    let file = "test_resources/arith.cl";
    let program = parse_program_from_file(file);
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  #[should_panic(expected = "Unexpected token: Dot { line_num: 5, line_pos: 54 }")]
  fn test_single_program_fail() {
    let file = "test_resources/cool_bad.cl";
    let program = parse_program_from_file(file);
    println!("{:#?}", program);
  }
}
