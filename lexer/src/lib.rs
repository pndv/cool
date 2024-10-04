use crate::nodes::{Class, Expression, Feature, Formal, ReadState, Program, Symbol, Type};
use crate::scanner::get_program_token_list;
use crate::tokens::Token;
use core::iter::Iterator;
use std::iter::{Filter, Peekable};
use std::process::id;
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
  let _ = match_required_token(token_option, Token::left_curl_type());

  let features = get_features(token_iter);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::right_curl_type());

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
    
    Some(peeked_token) if peeked_token.is_same_type(&Token::left_paren_type()) => 
      get_method_feature(ident_name, token_iter),

    Some(t) =>  panic!("Incorrect token {:?}", t),
    
    None => panic!("Unexpected EOF"),
    
  }
}

fn get_method_feature(ident_name: Symbol, token_iter: &mut FilteredTokensIterator) -> Feature {
  let mut token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::left_paren_type());

  let mut formals: Option<Vec<Formal>> = None;

  if token_iter.peek().is_some() && token_iter.peek().unwrap().is_same_type(&Token::right_paren_type()) {
    let vec_formals = get_formals(token_iter);
    formals = Some(vec_formals);
  }

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::right_paren_type());

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::colon_type());

  token_option = token_iter.next();
  let token = match_required_token(token_option, Token::ident_type());
  let method_return_type = Type::from(token);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::left_curl_type());

  let method_expr = get_expression(token_iter);

  token_option = token_iter.next();
  let _ = match_required_token(token_option, Token::right_curl_type());

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
    let _ = match_required_token(token_option, Token::random_assign_value());

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

fn get_expression(token_iter: &mut FilteredTokensIterator, state: ReadState) -> Expression {
  if token_iter.peek().is_some()  { panic!("Unexpected EOF") };
  
  match state {
    ReadState::ExpressionStart => {
      get_tokens_from_start_state
    }
    ReadState::IdentStarting => {}
    ReadState::LetIn => {}
    ReadState::CaseOf => {}
    ReadState::CaseEnd => {}
    ReadState::WhileLoop => {}
    ReadState::WhileEnd => {}
    ReadState::ConditionalThen => {}
    ReadState::ConditionalElse => {}
    ReadState::ConditionalEnd => {}
    ReadState::BinaryPlus => {}
    ReadState::BinaryMinus => {}
    ReadState::BinaryMultiply => {}
    ReadState::BinaryDivide => {}
    ReadState::BinaryLessThan => {}
    ReadState::BinaryLessThanOrEqual => {}
    ReadState::BinaryEqual => {}
  }
  
  match token {
    Token::Empty => panic!("Unexpected token {:?}", token),
    Token::Error { .. } => panic!("Unexpected token {:?}", token),
    Token::Comment { .. } => panic!("Unexpected token {:?}", token),
    Token::Int { value,.. } => Expression::Int {value},
    Token::Str { value,.. } => Expression::String {value},
    Token::True { .. } => Expression::Bool { value: true},
    Token::False { .. } => Expression::Bool { value: false},

    Token::New { .. } => {
      let Some(type_token) = token_iter.next() else { panic!("Unexpected EOF") };
      let type_name: Type = Type::from(type_token);
      
      Expression::New {type_name }
    }

    Token::IsVoid { .. } => {
      let expr = get_expression(token_iter, state);
      Expression::IsVoid { expr: Box::new(expr) }
          
    }
    Token::Tilde { .. } => {
      let expr = get_expression(token_iter, state);
      Expression::Negate { expr: Box::new(expr) }
    }
    Token::Not { .. } => {
      let expr = get_expression(token_iter, state);
      Expression::Not { expr: Box::new(expr) }
    },
    
    _ => {
      
    }

  }
  
  fn get_tokens_from_start_state(token_iter: &mut FilteredTokensIterator, state: ReadState) -> Expression {
    let Some(token) = token_iter.peek() else { panic!("Unexpected EOF") };

    match token {
      
      Token::Int {..} | Token::Str {..} | Token::True { .. } | Token::False { .. } => {
        let t: Token = token_iter.next().unwrap();
        Expression::from(t)
      },

      Token::New { .. } => {
        let _ = token_iter.next(); // consume `new`
        let Some(type_token) = token_iter.next() else { panic!("Unexpected EOF") }; // consume `type` i.e. Ident
        let type_name: Type = Type::from(type_token);

        Expression::New {type_name }
      }

      Token::IsVoid { .. } => {
        let _ = token_iter.next(); // consume `IsVoid`
        let expr = get_expression(token_iter, ReadState::ExpressionStart);
        Expression::IsVoid { expr: Box::new(expr) }

      }
      Token::Tilde { .. } => {
        let _ = token_iter.next(); // consume `~`
        let expr = get_expression(token_iter, ReadState::ExpressionStart);
        Expression::Negate { expr: Box::new(expr) }
      }
      
      Token::Not { .. } => {
        let _ = token_iter.next(); // consume `Not`
        let expr = get_expression(token_iter, ReadState::ExpressionStart);
        Expression::Not { expr: Box::new(expr) }
      },
      
      Token::Ident { .. } => {
        /*
        At top level 
        - ID <- Type
        - ID ( [[expr]]+ ) -- dispatch (static or dynamic)
        - ID
         */
        
        let ident = token_iter.next().unwrap(); // consume the identifier
        
        let peeked_token = token_iter.peek();
        if !peeked_token.is_some() {
          // Nothing after Identifier, return plain Ident expression
          return Expression::from(ident);
        }
        
        let Some(next_token) = peeked_token else { panic!("Unexpected EOF") };
        match next_token {
          Token::AssignValue {..} => {
            let _ = token_iter.next(); // consume `<-`
            let symbol: Symbol = Symbol::from(ident);
            let expr = get_expression(token_iter, ReadState::ExpressionStart);
            Expression::Assign {name: symbol, expr: Box::new(expr) }
          }
          Token::LParen {..} => {
            let function_name: Symbol = Symbol::from(ident);
            let _ = token_iter.next(); // consume `(`
            let param_list: Vec<Box<Expression>> = get_expr_list(token_iter);
            let _ = token_iter.next(); // consume `)`
            
            // self.`fn(params`
            Expression::Dispatch {
              expr: Box::from(Expression::SelfExpr), 
              name: function_name,
              param_list
            }
          }
          _ => Expression::from(ident),
        }
        
      },

      _ => Expression::NoExpr,

    }

    
  }
  
  /// Get list of parameters for function call (dynamic and static dispatch)
  fn get_expr_list(token_iter: &mut FilteredTokensIterator) -> Vec<Box<Expression>>{
    let mut expr_list: Vec<Box<Expression>> = Vec::new();
    let expr = get_expression(token_iter, ReadState::ExpressionStart);
    expr_list.push(Box::from(expr));
    
    while let Some(Token::Comma {..}) = token_iter.peek() {
      let _ = token_iter.next(); // consume the ','
      let expr = get_expression(token_iter, ReadState::ExpressionStart);
      expr_list.push(Box::from(expr));
    }
    
    // check for calls like `fn(,,,,)` or `fn(,1)` or `fn(1,2,) and throw error 
    if expr_list.len() > 1 && expr_list.contains(&Box::from(Expression::NoExpr)) {
      panic!("Incorrect expression list {:?}", expr_list);
    }
    
    expr_list
  }
  
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
