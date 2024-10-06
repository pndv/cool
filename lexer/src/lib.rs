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

pub fn analyse_lexical(file_path: &str) {
  let Ok(tokens) = get_program_token_list(file_path) else { panic!("Error reading file"); };

  if let Some(err) = check_tokens(&tokens) {
    panic!("{err}");
  }

  let is_not_comment: CommentFilter = is_not_comment;

  let mut token_iter: FilteredTokensIterator = tokens.into_iter()
                                                     .filter(is_not_comment)
                                                     .peekable();

  let mut program: Program = Program::new();

  while token_iter.peek().is_some() {
    let class = get_class(&mut token_iter);
    program.add_class(class);
  }
}

fn is_not_comment(token: &Token) -> bool {
  match token {
    Token::Comment { .. } => false,
    _ => true,
  }
}

fn get_class(token_iter: &mut FilteredTokensIterator) -> Class {
  // guaranteed to be non-empty at the start
  let mut token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::class_type());

  token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::ident_type());
  let class_type: Type = Type::from(token);
  let mut parent_type: Option<Type> = None;

  token_option = token_iter.next();
  if token_option.is_some() && token_option.unwrap().is_same_type(&Token::inherits_type()) {
    token_option = token_iter.next();
    token = match_required_token(token_option, Token::ident_type());
    let inherits_from: Type = Type::from(token);
    parent_type = Some(inherits_from);
  }

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::open_curl_type());

  let features = get_features(token_iter);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::close_curl_type());

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::semi_colon_type());

  Class::new(class_type, parent_type, features)
}

fn get_features(token_iter: &mut FilteredTokensIterator) -> Option<Vec<Feature>> {
  let peek = token_iter.peek();
  if peek.is_none() || peek.unwrap() != &Token::semi_colon_type() {
    return None;
  }

  let mut features = Vec::new();
  let mut feature = get_feature(token_iter);
  features.push(feature);

  while token_iter.peek() == Some(&Token::semi_colon_type()) {
    match_required_token(token_iter.next(), Token::semi_colon_type()); // Consume ','

    feature = get_feature(token_iter);
    features.push(feature);
  }

  Some(features)
}

fn get_feature(token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let token = match_required_token(token_option, Token::ident_type());

  let ident_name: Symbol = Symbol::from(token);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::semi_colon_type());

  match token_iter.peek() {
    Some(peeked_token) if peeked_token.is_same_type(&Token::colon_type()) =>
      get_attribute_feature(ident_name, token_iter),

    Some(peeked_token) if peeked_token.is_same_type(&Token::open_paren_type()) =>
      get_method_feature(ident_name, token_iter),

    Some(t) => panic!("Incorrect token {:?}", t),

    None => panic!("Unexpected EOF"),
  }
}

fn get_method_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::open_paren_type());

  let mut formals: Option<Vec<Formal>> = None;

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::close_paren_type()) {
    let vec_formals = get_formals(token_iter);
    formals = Some(vec_formals);
  }

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::close_paren_type());

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::colon_type());

  token_option = token_iter.next();
  let token = match_required_token(token_option, Token::ident_type());
  let method_return_type = Type::from(token);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::open_curl_type());

  let method_expr = get_expression(token_iter);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::close_curl_type());

  (ident_name, formals, method_return_type, Box::from(method_expr)).into()
}

fn get_attribute_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::colon_type());

  token_option = token_iter.next();
  let token = match_required_token(token_option, Token::ident_type());
  let method_return_type = Symbol::from(token);

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::colon_type()) {
    token_option = token_iter.next();
    let _ = match_required_token(token_option, Token::assign_type());

    let method_expr = get_expression(token_iter);
    let feature = (ident_name, method_return_type, Box::from(method_expr)).into();
    feature
  } else {
    (ident_name, method_return_type).into()
  }
}

fn get_formals(token_iter: &mut FilteredTokensIterator) -> Vec<Formal> {
  let mut formals = Vec::new();
  let mut formal = get_formal(token_iter);
  formals.push(formal);

  while token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::comma_type()) {
    match_required_token(token_iter.next(), Token::comma_type()); // Consume ','

    formal = get_formal(token_iter);
    formals.push(formal);
  }

  formals
}

fn get_formal(token_iter: &mut FilteredTokensIterator) -> Formal {
  let mut token_option = token_iter.next();
  let mut token = match_required_token(token_option, Token::ident_type());

  let formal_name: Symbol = Symbol::from(token);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::colon_type()); // consume colon

  token_option = token_iter.next();
  token = match_required_token(token_option, Token::ident_type());
  let formal_type: Type = Type::from(token);

  (formal_name, formal_type).into()
}

fn get_expression(token_iter: &mut FilteredTokensIterator) -> Expression {
  
  if token_iter.peek().is_some() { panic!("Unexpected EOF") };
  let mut stack: Vec<Box<Expression>> = Vec::new();
  fill_expression_stack(token_iter, &mut stack);
  
  // The entire stack must reduce to a single `Expression`
  if stack.is_empty() { return Expression::NoExpr; }
  if stack.len() == 1 { return *stack[0]; }

  for i in 0..stack.len() {
    
  }
    
  
}



fn fill_expression_stack(token_iter: &mut FilteredTokensIterator, stack: &mut Vec<Box<Expression>>) {
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

      (ident_name, ident_type, expr).into()
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

      (ident_name, ident_type, expr).into()
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


  let Some(token) = token_iter.peek() else { panic!("Unexpected EOF") };
  let mut expr = Expression::NoExpr;
  match token {
    Token::Ident { .. } => {
      /*
      At top level 
      - ID <- Type
      - ID ( [[expr]]+ ) -- dispatch (static or dynamic)
      - ID
       */

      let ident = match_required_token(token_iter.next(), Token::ident_type()); // consume the identifier

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
    }

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

    Token::If { .. } => {
      match_required_token(token_iter.next(), Token::if_type()); // consume `If`
      let predicate = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), Token::then_type()); // consume `Then`
      let then_expr = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), Token::else_type()); // consume `Else`
      let else_expr = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), Token::end_if_type()); // consume `Fi`

      expr = Expression::Conditional { predicate, then_expr, else_expr };
    }

    Token::While { .. } => {
      let _ = token_iter.next(); // consume `while`

      let predicate = get_stack_top(token_iter, stack);
      match_required_token(token_iter.next(), Token::loop_type());

      let body = get_stack_top(token_iter, stack);
      match_required_token(token_iter.next(), Token::end_loop_type());

      expr = Expression::Loop { predicate, body };
    }

    Token::OpenCurl { .. } => {
      let _ = token_iter.next(); // consume `{`

      let expressions = get_block_expr_list(token_iter, stack);

      match_required_token(token_iter.next(), Token::close_curl_type()); // consume `}`

      expr = Expression::Block { expr_list: expressions };
    }

    Token::Let { .. } => {
      let _ = token_iter.next(); // consume `let`
      let expr_list = get_let_init_list(token_iter, stack);

      match_required_token(token_iter.next(), Token::in_type()); // consume `in`
      let in_expr = get_stack_top(token_iter, stack);

      expr = Expression::Let { let_init: expr_list, in_expr };
    }

    Token::Case { .. } => {
      let _ = token_iter.next(); // consume `case`

      let switch_expression = get_stack_top(token_iter, stack);

      match_required_token(token_iter.next(), Token::of_type()); // consume `of`

      let branches = get_case_branch_list(token_iter, stack);

      match_required_token(token_iter.next(), Token::end_case_type()); // consume `esac` 

      expr = Expression::Case { switch_expression, branches };
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

  stack.push(Box::from(expr))
}

fn match_required_token(token_option: Option<Token>, expected: Token) -> Token {
  if let Some(token) = token_option {
    if !token.is_same_type(&expected) {
      panic!("Unexpected token: {:?}", token);
    }

    token
  } else {
    panic!("Unexpected EOF");
  }
}

fn check_tokens(tokens: &Vec<Token>) -> Option<String> {
  let mut errors: String = String::from("");
  for token in tokens {
    match token {
      Token::Empty => {
        errors.push_str("Empty token! Parsing failed somewhere, can't specify details.\n");
      }
      Token::Error { error_char, line_num, line_pos } => {
        let x = format!("Error on line {line_num} at pos {line_pos}, offending character {error_char}.");
        errors.push_str(x.as_str())
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
