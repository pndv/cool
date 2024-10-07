use crate::nodes::{CaseBranch, Class, Expression, Feature, Formal, LetInit, Program, Symbol, Type};
use crate::scanner::get_program_token_list;
use crate::tokens::Token;
use core::iter::Iterator;
use std::iter::{Filter, Peekable};
use std::vec::IntoIter;

pub mod scanner;
pub mod nodes;
pub mod tokens;

type CommentFilter = fn(&Token) -> bool;
type FilteredTokensIterator = Peekable<Filter<IntoIter<Token>, CommentFilter>>;

#[must_use]
pub fn parse_program_from_file(file_path: &str) -> Program {
  let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);

  get_program(&mut token_iter)
}

fn get_filtered_token_iter(file_path: &str) -> FilteredTokensIterator {
  let Ok(tokens) = get_program_token_list(file_path) else { panic!("Error reading file"); };

  if let Some(err) = check_tokens(&tokens) {
    panic!("{err}");
  }

  let is_not_comment: CommentFilter = is_not_comment;

  let token_iter: FilteredTokensIterator = tokens.into_iter()
                                                     .filter(is_not_comment)
                                                     .peekable();

  token_iter
}

fn is_not_comment(token: &Token) -> bool {
  !matches!(token, Token::Comment { .. })
}

fn get_program(token_iter: &mut FilteredTokensIterator) -> Program {
  let mut program: Program = Program::new();

  while token_iter.peek().is_some() {
    let class = get_class(token_iter);
    program.add_class(class);
  }

  program
}

fn get_class(token_iter: &mut FilteredTokensIterator) -> Class {
  match_required_token(token_iter.next(), Token::class_type());

  let mut token = match_required_token(token_iter.next(), Token::ident_type());
  let class_type: Type = Type::from(token);

  let mut parent_type: Option<Type> = None;
  let peeked_token = token_iter.peek();
  if peeked_token.is_some() && peeked_token.unwrap().is_same_type(&Token::inherits_type()) {
    match_required_token(token_iter.next(), Token::inherits_type());

    token = match_required_token(token_iter.next(), Token::ident_type());
    let inherits_from: Type = Type::from(token);
    parent_type = Some(inherits_from);
  }

  match_required_token(token_iter.next(), Token::open_curl_type());
  
  let mut features: Option<Vec<Feature>> = None;
  if token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&Token::close_curl_type()) {
    features = get_features(token_iter);
  }
  
  match_required_token(token_iter.next(), Token::close_curl_type());
  match_required_token(token_iter.next(), Token::semi_colon_type());

  Class::new(class_type, parent_type, features)
}

/// Features :-> feature; {{ features }}
fn get_features(token_iter: &mut FilteredTokensIterator) -> Option<Vec<Feature>> {

  let mut features = Vec::new();
  let mut feature = get_feature(token_iter);
  features.push(feature);

  // read till closing `}` encountered for `class` 
  while token_iter.peek().is_some() && !token_iter.peek()?.is_same_type(&Token::close_curl_type()) {
    feature = get_feature(token_iter);
    features.push(feature);
  }

  Some(features)
}

fn get_feature(token_iter: &mut FilteredTokensIterator) -> Feature {
  //Feature starts with ID
  let token = match_required_token(token_iter.next(), Token::ident_type());
  let ident_name: Symbol = Symbol::from(token);

  let feature: Feature = match token_iter.peek() {
    Some(peeked_token) if peeked_token.is_same_type(&Token::colon_type()) =>
      get_attribute_feature(ident_name, token_iter),

    Some(peeked_token) if peeked_token.is_same_type(&Token::open_paren_type()) =>
      get_method_feature(ident_name, token_iter),

    Some(t) => panic!("Incorrect token {:?}", t),

    None => panic!("Unexpected EOF"),
  };

  // Feature must terminate with a semicolon
  match_required_token(token_iter.next(), Token::semi_colon_type());

  feature
}

fn get_method_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  match_required_token(token_iter.next(), Token::open_paren_type());

  let mut formals: Option<Vec<Formal>> = None;

  // If the next token is not `)`, read formals list
  if token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&Token::close_paren_type()) {
    let vec_formals = get_formals(token_iter);
    formals = Some(vec_formals);
  }

  match_required_token(token_iter.next(), Token::close_paren_type());

  match_required_token(token_iter.next(), Token::colon_type());

  let token = match_required_token(token_iter.next(), Token::ident_type());
  let method_return_type = Type::from(token);

  match_required_token(token_iter.next(), Token::open_curl_type());

  let method_expr = get_expression(token_iter);

  match_required_token(token_iter.next(), Token::close_curl_type());

  (ident_name, formals, method_return_type, Box::from(method_expr)).into()
}

fn get_attribute_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  match_required_token(token_iter.next(), Token::colon_type());

  let token = match_required_token(token_iter.next(), Token::ident_type());
  let method_return_type = Symbol::from(token);

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::colon_type()) {
    match_required_token(token_iter.next(), Token::assign_type());

    let method_expr = get_expression(token_iter);
    (ident_name, method_return_type, Box::from(method_expr)).into()
  } else {
    (ident_name, method_return_type).into()
  }
}

/// Formals |-> formal {{, formals}}
fn get_formals(token_iter: &mut FilteredTokensIterator) -> Vec<Formal> {

  let mut formals: Vec<Formal> = Vec::new();
  let mut formal = get_formal(token_iter);
  formals.push(formal);

  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::comma_type()) {
    match_required_token(token_iter.next(), Token::comma_type()); // Consume ','

    formal = get_formal(token_iter);
    formals.push(formal);
  }

  formals
}

/// Formal |-> ID : TYPE
fn get_formal(token_iter: &mut FilteredTokensIterator) -> Formal {
  
  let mut token = match_required_token(token_iter.next(), Token::ident_type());
  let formal_name: Symbol = Symbol::from(token);

  match_required_token(token_iter.next(), Token::colon_type()); // consume colon

  token = match_required_token(token_iter.next(), Token::ident_type());
  let formal_type: Type = Type::from(token);

  (formal_name, formal_type).into()
}

fn get_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  assert!(token_iter.peek().is_some(), "Unexpected EOF"); // assert that there are some tokens remaining

  let mut stack: Vec<Box<Expression>> = Vec::new();
  fill_expression_stack(token_iter, &mut stack);

  // The entire stack must reduce to a single `Expression`
  if stack.is_empty() { return Expression::NoExpr; }

  let mut expr = Expression::NoExpr;

  if stack.len() == 1 {
    expr = (*stack[0]).clone();

    // If first expression is a PartialDispatch convert it to Dispatch by adding `self`
    if let Expression::PartialDispatch { fn_name, param_list } = expr {
      expr = Expression::Dispatch {
        calling_expr: Box::from(Expression::SelfExpr),
        cast_type_name: None,
        fn_name,
        param_list,
      }
    }

    return expr;
  }

  for i in 1..stack.len() {
    let next_expr = *stack[i].clone();
    assert!(next_expr.is_partial());

    match next_expr {
      Expression::PartialBinary { binary_token, right_expr } => {
        match binary_token {
          Token::Star { .. } => expr = Expression::Multiply { left: stack[0].clone(), right: right_expr },
          Token::ForwardSlash { .. } => expr = Expression::Divide { left: stack[0].clone(), right: right_expr },
          Token::Plus { .. } => expr = Expression::Plus { left: stack[0].clone(), right: right_expr },
          Token::Minus { .. } => expr = Expression::Minus { left: stack[0].clone(), right: right_expr },
          Token::LessOrEqual { .. } => expr = Expression::LessThanOrEqual { left: stack[0].clone(), right: right_expr },
          Token::Less { .. } => expr = Expression::LessThan { left: stack[0].clone(), right: right_expr },
          Token::Equal { .. } => expr = Expression::Equal { left: stack[0].clone(), right: right_expr },
          _ => panic!("Not a binary token {:?}", binary_token),
        }
      }

      Expression::PartialDispatch { fn_name, param_list } => {
        let first = stack[0].clone();
        expr = Expression::Dispatch { calling_expr: first, cast_type_name: None, fn_name, param_list }
      }
      Expression::PartialCastDispatch { cast_type, partial_dispatch } => {
        let first = stack[0].clone();
        if let Expression::PartialDispatch { fn_name, param_list } = *partial_dispatch {
          expr = Expression::Dispatch { calling_expr: first, cast_type_name: Some(cast_type), fn_name, param_list }
        } else {
          panic!("PartialCast does not have correct dispatch expression {:?}", partial_dispatch);
        }
      }

      _ => panic!("Should be partial expression {:?}", next_expr)
    }
  }

  expr
}

fn fill_expression_stack(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) {
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
      let ident_type = match_required_token(token_iter.next(), Token::ident_type()); // consume `TYPE`
      let cast_type = Type::from(ident_type);

      // `.` must follow the cast_type
      assert!(token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::dot_type()));

      let dispatch_expr = get_stack_top(token_iter, stack);
      if let Expression::PartialDispatch { .. } = *dispatch_expr {
        expr = Expression::PartialCastDispatch { cast_type, partial_dispatch: dispatch_expr }
      } else {
        panic!("Invalid state for dynamic dispatch expression. Expected PartialDispatch type found {:?}", dispatch_expr);
      }
    }

    Token::Dot { .. } => {
      match_required_token(token_iter.next(), Token::dot_type()); // consume `.`

      // `Ident` must follow the `.`
      assert!(token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::ident_type()));

      let dispatch_expr = get_stack_top(token_iter, stack);
      if let Expression::PartialDispatch { .. } = *dispatch_expr {
        expr = *dispatch_expr;
      } else {
        panic!("Invalid state for dynamic dispatch expression. Expected PartialDispatch type found {:?}", dispatch_expr);
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
      expr = Expression::IsVoid { expr: void_expr };
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
      expr = Expression::PartialBinary { binary_token, right_expr };
    }

    Token::Tilde { .. } => {
      let _ = token_iter.next(); // consume `~`
      let tilde_expr = get_stack_top(token_iter, stack);
      expr = Expression::Negate { expr: tilde_expr };
    }

    Token::Not { .. } => {
      let _ = token_iter.next(); // consume `Not`
      let not_expr = get_stack_top(token_iter, stack);
      expr = Expression::Not { expr: not_expr };
    }

    Token::OpenParen { .. } => {
      let _ = token_iter.next(); // consume `(`
      let e = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), Token::close_paren_type());
      expr = *e;
    }

    Token::Int { .. } | Token::Str { .. } | Token::True { .. } | Token::False { .. } => {
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

  stack.push(Box::from(expr));
}

fn get_case_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  let _ = token_iter.next(); // consume `case`

  let switch_expression = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), Token::of_type()); // consume `of`

  let branches = get_case_branch_list(token_iter, stack);

  match_required_token(token_iter.next(), Token::end_case_type()); // consume `esac` 

  Expression::Case { switch_expression, branches }
}

fn get_let_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  let _ = token_iter.next(); // consume `let`
  let expr_list = get_let_init_list(token_iter, stack);

  match_required_token(token_iter.next(), Token::in_type()); // consume `in`
  let in_expr = get_stack_top(token_iter, stack);

  Expression::Let { let_init: expr_list, in_expr }
}

fn get_block_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  let _ = token_iter.next(); // consume `{`

  let expressions = get_block_expr_list(token_iter, stack);

  match_required_token(token_iter.next(), Token::close_curl_type()); // consume `}`

  Expression::Block { expr_list: expressions }
}

fn get_loop_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  match_required_token(token_iter.next(), Token::while_type()); // consume `while`

  let predicate = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), Token::loop_type());

  let body = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), Token::end_loop_type());

  Expression::Loop { predicate, body }
}

fn get_conditional_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  match_required_token(token_iter.next(), Token::if_type()); // consume `If`
  let predicate = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), Token::then_type()); // consume `Then`
  let then_expr = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), Token::else_type()); // consume `Else`
  let else_expr = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), Token::end_if_type()); // consume `Fi`

  Expression::Conditional { predicate, then_expr, else_expr }
}

/// The expression from ID could be:
/// 1. ID <- Type
/// 2. ID ( {{expr}}+ ) -- dispatch (static or dynamic)
/// 3. ID
fn get_expr_from_ident(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Expression {
  let ident = match_required_token(token_iter.next(), Token::ident_type()); // consume the identifier
  let expr: Expression;

  let peeked_token = token_iter.peek();

  if let Some(next_token) = peeked_token {
    match next_token {
      Token::Assign { .. } => {
        let _ = token_iter.next(); // consume `<-`
        let symbol: Symbol = Symbol::from(ident);
        let assign_expr = get_stack_top(token_iter, stack);
        expr = Expression::Assign { name: symbol, expr: assign_expr }
      }
      Token::OpenParen { .. } => {
        let _ = token_iter.next(); // consume `(`

        let function_name: Symbol = Symbol::from(ident);
        let param_list: Vec<Box<Expression>> = get_fn_param_expr_list(token_iter, stack);

        match_required_token(token_iter.next(), Token::close_paren_type()); // consume `)`

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

fn get_stack_top(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Box<Expression> {
  let count = stack.len();
  fill_expression_stack(token_iter, stack);
  assert_eq!(stack.len(), count + 1); // should have only one more expression on the stack
  stack.pop().unwrap()
}

fn get_block_expr_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Vec<Box<Expression>> {
  let mut block_expr_list: Vec<Box<Expression>> = Vec::new();

  let expr = get_stack_top(token_iter, stack);
  match_required_token(token_iter.next(), Token::semi_colon_type());
  block_expr_list.push(expr);

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&Token::close_curl_type()) {
    let expr = get_stack_top(token_iter, stack);
    match_required_token(token_iter.next(), Token::semi_colon_type());
    block_expr_list.push(expr);
  }

  block_expr_list
}

fn get_let_init_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Vec<LetInit> {
  /// matches `Id:Type [[ <- Expression ]]`
  fn get_let_init(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> LetInit {
    let ident_token = match_required_token(token_iter.next(), Token::ident_type());
    let ident_name: Symbol = Symbol::from(ident_token);

    match_required_token(token_iter.next(), Token::colon_type());

    let ident_type_token = match_required_token(token_iter.next(), Token::ident_type());
    let ident_type: Type = Type::from(ident_type_token);

    let mut expr: Option<Box<Expression>> = None;
    let peek = token_iter.peek();
    if peek.is_some() && peek.unwrap().is_same_type(&Token::assign_type()) {
      match_required_token(token_iter.next(), Token::assign_type()); // consume `<-`
      expr = Some(get_stack_top(token_iter, stack));
    }

    (ident_name, ident_type, expr)
  }

  let mut list: Vec<LetInit> = Vec::new();

  let first_init = get_let_init(token_iter, stack);
  list.push(first_init);

  // get `LetInit` while there is `Comma`
  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::comma_type()) {
    let init = get_let_init(token_iter, stack);
    list.push(init);
  }

  list
}

/// Get list of all branches of `case` statement
fn get_case_branch_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Vec<CaseBranch> {
  /// matches `Id:Type => Expression;`
  fn get_case_branch(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> CaseBranch {
    let ident_token = match_required_token(token_iter.next(), Token::ident_type());
    let ident_name: Symbol = Symbol::from(ident_token);

    match_required_token(token_iter.next(), Token::colon_type());

    let type_token = match_required_token(token_iter.next(), Token::ident_type());
    let ident_type: Type = Type::from(type_token);

    match_required_token(token_iter.next(), Token::lambda_type());
    let expr = get_stack_top(token_iter, stack);

    match_required_token(token_iter.next(), Token::semi_colon_type());

    (ident_name, ident_type, expr)
  }

  let mut case_branch_list: Vec<CaseBranch> = Vec::new();

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&Token::end_case_type()) {
    case_branch_list.push(get_case_branch(token_iter, stack));
  }

  case_branch_list
}

/// Get list of parameters for function call (dynamic and static dispatch)
fn get_fn_param_expr_list(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) -> Vec<Box<Expression>> {
  let mut expr_list: Vec<Box<Expression>> = Vec::new();

  while token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&Token::close_paren_type()) {
    let expr = get_stack_top(token_iter, stack);
    expr_list.push(expr);

    if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::comma_type()) {
      // consume `,` continue with the loop
      match_required_token(token_iter.next(), Token::comma_type());
    }
  }

  expr_list
}

fn match_required_token(token_option: Option<Token>, expected: Token) -> Token {
  if let Some(token) = token_option {
    assert!(token.is_same_type(&expected), "Unexpected token: {:?}", token);
    token
  } else {
    panic!("Unexpected EOF");
  }
}

fn check_tokens(tokens: &Vec<Token>) -> Option<String> {
  let mut errors: String = String::new();
  for token in tokens {
    match token {
      Token::Empty => {
        errors.push_str("Empty token! Parsing failed somewhere, can't specify details.\n");
      }
      Token::Error { error_char, line_num, line_pos } => {
        let x = format!("Error on line {line_num} at pos {line_pos}, offending character {error_char}.");
        errors.push_str(x.as_str());
      }
      _ => continue,
    }
  }

  if errors.is_empty() {
    None
  } else {
    Some(errors)
  }
}

mod test {
  use crate::tokens::Token;
  use crate::{get_filtered_token_iter, parse_program_from_file};
  use std::mem::discriminant;

  #[test]
  fn test_filtered_token_iter() {
    let file = "test_resources/cool.cl";
    let iter = get_filtered_token_iter(file);
    for token in iter {
      assert_ne!(discriminant(&token), discriminant(&Token::Comment { value: String::new(), line_num: 0, line_pos: 0 }));
      println!("{:?}", token);
    }
  }

  #[test]
  fn test_single_program() {
    let file = "test_resources/cool.cl";
    let program = parse_program_from_file(file);
    println!("{:#?}", program);
  }

  #[test]
  #[should_panic(expected = "Unexpected token: Dot { line_num: 5, line_pos: 54 }")]
  fn test_single_program_fail() {
    let file = "test_resources/cool_bad.cl";
    let program = parse_program_from_file(file);
    println!("{:#?}", program);
  }
}
