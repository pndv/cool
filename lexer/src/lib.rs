use crate::nodes::{CaseBranch, Class, Expression, Feature, Formal, LetInit, Program, Symbol, Type};
use crate::scanner::get_program_token_list;
use crate::tokens::{Token, ASSIGN_TYPE, CASE_TYPE, CLASS_TYPE, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, COLON_TYPE, COMMA_TYPE, DOT_TYPE, ELSE_TYPE, END_CASE_TYPE, END_IF_TYPE, END_LOOP_TYPE, IDENT_TYPE, IF_TYPE, INHERITS_TYPE, IN_TYPE, LAMBDA_TYPE, LET_TYPE, LOOP_TYPE, OF_TYPE, OPEN_CURL_TYPE, OPEN_PAREN_TYPE, SEMI_COLON_TYPE, THEN_TYPE, WHILE_TYPE};
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

  convert_vec_filtered_iter(tokens)
}

fn convert_vec_filtered_iter(tokens: Vec<Token>) -> FilteredTokensIterator {
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
  match_required_token(token_iter.next(), CLASS_TYPE);

  let mut token = match_required_token(token_iter.next(), IDENT_TYPE);
  let class_type: Type = Type::from(token);

  let mut parent_type: Option<Type> = None;
  let peeked_token = token_iter.peek();
  if peeked_token.is_some() && peeked_token.unwrap().is_same_type(&INHERITS_TYPE) {
    match_required_token(token_iter.next(), INHERITS_TYPE);

    token = match_required_token(token_iter.next(), IDENT_TYPE);
    let inherits_from: Type = Type::from(token);
    parent_type = Some(inherits_from);
  }

  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let mut features: Option<Vec<Feature>> = None;
  if token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&CLOSE_CURL_TYPE) {
    features = get_features(token_iter);
  }

  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);
  match_required_token(token_iter.next(), SEMI_COLON_TYPE);

  Class::new(class_type, parent_type, features)
}

/// Features :-> feature; {{ features }}
fn get_features(token_iter: &mut FilteredTokensIterator) -> Option<Vec<Feature>> {
  let mut features = Vec::new();
  let mut feature = get_feature(token_iter);
  features.push(feature);

  // `{` seen in calling method => read till closing `}` encountered for `class` 
  while token_iter.peek().is_some() && !token_iter.peek()?.is_same_type(&CLOSE_CURL_TYPE) {
    feature = get_feature(token_iter);
    features.push(feature);
  }

  Some(features)
}

fn get_feature(token_iter: &mut FilteredTokensIterator) -> Feature {
  //Feature starts with ID
  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let ident_name: Symbol = Symbol::from(token);

  let feature: Feature = match token_iter.peek() {
    Some(peeked_token) if peeked_token.is_same_type(&COLON_TYPE) =>
      get_attribute_feature(ident_name, token_iter),

    Some(peeked_token) if peeked_token.is_same_type(&OPEN_PAREN_TYPE) =>
      get_method_feature(ident_name, token_iter),

    Some(t) => panic!("Incorrect token {:?}", t),

    None => panic!("Unexpected EOF"),
  };

  // Feature must terminate with a semicolon
  match_required_token(token_iter.next(), SEMI_COLON_TYPE);

  feature
}

fn get_method_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  match_required_token(token_iter.next(), OPEN_PAREN_TYPE);

  let mut formals: Option<Vec<Formal>> = None;

  // `(` seen in calling method => If the next token is not `)`, read formals list
  if token_iter.peek().is_some() && !token_iter.peek().unwrap().is_same_type(&CLOSE_PAREN_TYPE) {
    let vec_formals = get_formals(token_iter);
    formals = Some(vec_formals);
  }

  match_required_token(token_iter.next(), CLOSE_PAREN_TYPE);

  match_required_token(token_iter.next(), COLON_TYPE);

  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let method_return_type = Type::from(token);

  match_required_token(token_iter.next(), OPEN_CURL_TYPE);

  let method_expr = get_expression(token_iter);

  match_required_token(token_iter.next(), CLOSE_CURL_TYPE);

  (ident_name, formals, method_return_type, Box::from(method_expr)).into()
}

fn get_attribute_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  match_required_token(token_iter.next(), COLON_TYPE);

  let token = match_required_token(token_iter.next(), IDENT_TYPE);
  let method_return_type = Symbol::from(token);

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&COLON_TYPE) {
    match_required_token(token_iter.next(), ASSIGN_TYPE);

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
  let formal_name: Symbol = Symbol::from(token);

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
        cast_type_name: None,
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
        expr = Expression::Dispatch { calling_expr: Box::from(first), cast_type_name: None, fn_name, param_list }
      }
      Expression::PartialCastDispatch { cast_type, partial_dispatch } => {
        let first = stack[0].clone();
        if let Expression::PartialDispatch { fn_name, param_list } = *partial_dispatch {
          expr = Expression::Dispatch { calling_expr: Box::from(first), cast_type_name: Some(cast_type), fn_name, param_list }
        } else {
          panic!("PartialCast does not have correct dispatch expression {:?}", partial_dispatch);
        }
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
      let cast_type = Type::from(ident_type);

      // `.` must follow the cast_type
      assert!(token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&DOT_TYPE));

      let dispatch_expr = get_stack_top(token_iter, stack);
      if let Expression::PartialDispatch { .. } = dispatch_expr {
        expr = Expression::PartialCastDispatch { cast_type, partial_dispatch: Box::from(dispatch_expr) }
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

  stack.push(expr);
}

fn get_case_expr(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Expression>) -> Expression {
  match_required_token(token_iter.next(), CASE_TYPE); // consume `case`

  let switch_expression = get_stack_top(token_iter, stack);

  match_required_token(token_iter.next(), OF_TYPE); // consume `of`

  let branches = get_case_branch_list(token_iter, stack);

  match_required_token(token_iter.next(), END_CASE_TYPE); // consume `esac` 

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
        let symbol: Symbol = Symbol::from(ident);
        let assign_expr = get_stack_top(token_iter, stack);
        expr = Expression::Assign { name: symbol, expr: Box::from(assign_expr) }
      }
      Token::OpenParen { .. } => {
        let _ = token_iter.next(); // consume `(`

        let function_name: Symbol = Symbol::from(ident);
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
    let ident_name: Symbol = Symbol::from(ident_token);

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
    let ident_name: Symbol = Symbol::from(ident_token);

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
  use crate::{get_features, get_filtered_token_iter, parse_program_from_file, FilteredTokensIterator};
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
  fn test_large_program() {
    let file = "test_resources/arith.cl";
    let program = parse_program_from_file(file);
    assert!(program.classes().len() > 1);
    assert_eq!(program.classes().len(), 7);
    println!("{:#?}", program.classes().len());
  }

  #[test]
  fn test_method_feature() {
    let file_path = "test_resources/feature.test";
    let mut token_iter: FilteredTokensIterator = get_filtered_token_iter(file_path);
    let features = get_features(&mut token_iter);
    println!("{:#?}", features);
  }
  
  #[test]
  #[should_panic(expected = "Unexpected token: Dot { line_num: 5, line_pos: 54 }")]
  fn test_single_program_fail() {
    let file = "test_resources/cool_bad.cl";
    let program = parse_program_from_file(file);
    println!("{:#?}", program);
  }
}
