use crate::models::symbols::{EnvType, Symbol, SymbolTable, SymbolType};
use lexer::model::constants::KEYWORD_SELF_TYPE;
use parser::model::class::ParseClass;
use parser::model::class::{BOOL_CLASS_NAME, INT_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME, STR_CLASS_NAME};
use parser::model::expressions::Expression;
use parser::model::feature::{Attribute, Method, ParseFeature};
use parser::model::program::ParseProgram;

fn gen_symbol_table(program: ParseProgram) -> Result<SymbolTable, String> {
  let mut symbol_table = SymbolTable::new();
  symbol_table.enter_scope();

  fill_built_in_classes(&mut symbol_table);

  // fill the symbol table with all the class names
  for class in program.classes() {
    let class_symbol: Symbol = Symbol { name: class.name.get_name(), env_type: EnvType::Program, sym_type: "Class" };
    symbol_table.put(class_symbol)
  }

  for class in program.classes {
    fill_symbol_table_class(&class, &mut symbol_table)?;
  }

  Ok(symbol_table)
}

fn fill_built_in_classes(symbol_table: &mut SymbolTable) {
  symbol_table.put(Symbol { name: OBJECT_CLASS_NAME.to_string(), env_type: EnvType::Program, sym_type: String:: from("Class"), ret_type: String::from("Void") });
  symbol_table.put(Symbol { name: IO_CLASS_NAME.to_string(), env_type: EnvType::Program, sym_type: String:: from("Class"), ret_type: String::from("Void") });
  symbol_table.put(Symbol { name: INT_CLASS_NAME.to_string(), env_type: EnvType::Program, sym_type: String:: from("Class"), ret_type: String::from("Void") });
  symbol_table.put(Symbol { name: STR_CLASS_NAME.to_string(), env_type: EnvType::Program, sym_type: String:: from("Class"), ret_type: String::from("Void") });
  symbol_table.put(Symbol { name: BOOL_CLASS_NAME.to_string(), env_type: EnvType::Program, sym_type: String:: from("Class"), ret_type: String::from("Void") });
}

fn fill_symbol_table_class(class: &ParseClass, symbol_table: &mut SymbolTable) -> Result<(), String> {
  symbol_table.enter_scope();
  let mut error_message = String::new();

  let parent_name_binding = class.parent_type.get_name();
  let parent_name = parent_name_binding.as_str();
  if !symbol_table.lookup_symbol_by_env_type(parent_name, EnvType::Class) {
    error_message.push_str(format!("Class {parent_name} does not exist").as_str());
  }

  if class.features.is_none() {
    return if error_message.is_empty() {
      Ok(())
    } else {
      Err(error_message)
    };
  }

  let self_type = Symbol { name: class.get_name(), env_type: EnvType::Class, sym_type: String::from("Self"), ret_type: String::from("Void") };
  let features = class.features.as_ref().unwrap();

  // First, populate all class-level features
  for feature in features {
    match feature {
      ParseFeature::Attribute { attribute } => {
        let attribute_name = attribute.name;
        let ret_type = attribute.return_type.get_name();

        if !symbol_table.lookup_symbol_by_env_type(&ret_type, EnvType::Class) {
          error_message.push_str(format!("Class {}: attribute {attribute_name} with return type {ret_type} does not exist", class.get_name()).as_str());
        }

        let attribute_symbol = Symbol { name: attribute_name, env_type: EnvType::Class, sym_type: SymbolType::Ident { ident_type: ret_type } };
        symbol_table.put(attribute_symbol);
      }

      ParseFeature::Method { method } => {
        let ret_type = if method.return_type.get_name() != KEYWORD_SELF_TYPE {
          method.return_type.get_name()
        } else {
          class.get_name()
        };

        // Check if the return type is in the symbol table
        if !symbol_table.lookup_symbol_by_env_type(&ret_type, EnvType::Class) {
          error_message.push_str(format!("Class {}: method {} with return type {ret_type} does not exist", class.get_name(), method.get_name()).as_str());
        }

        let method_symbol = Symbol { name: method.get_name(), env_type: EnvType::Class, sym_type: SymbolType::Ident { ident_type: ret_type } };
        symbol_table.put(method_symbol);
      }
    }
  }

  // Continue with the semantic checking each of the methods
  for feature in features {
    match feature {
      ParseFeature::Attribute { attribute } => {
        match fill_symbol_table_attribute(attribute, symbol_table) {
          Ok(_) => {}
          Err(err) => { error_message.push_str(err.as_str()) }
        }
      }
      ParseFeature::Method { method } => {
        match fill_symbol_table_method(method, symbol_table) {
          Ok(_) => {}
          Err(err) => error_message.push_str(err.as_str()),
        }
      }
    }
  }

  symbol_table.exit_scope();

  if error_message.is_empty() {
    Ok(())
  } else {
    Err(error_message)
  }
}

fn fill_symbol_table_attribute(attribute: &Attribute, symbol_table: &mut SymbolTable) -> Result<(), String> {
  let mut error_message = String::new();

  let attr_name = attribute.get_name();
  let attr_ret_type = attribute.return_type.get_name();

  if !symbol_table.lookup_symbol_by_env_type(&attr_ret_type, EnvType::Class) {
    error_message.push_str(format!("Return type {attr_ret_type} not defined for attribute {attr_name}").as_str());
  }

  symbol_table.put(Symbol { name: attr_name, env_type: EnvType::Class, sym_type: SymbolType::Ident { ident_type: attr_ret_type } });

  if attribute.expr.is_none() {
    if error_message.is_empty() {
      return Ok(());
    } else {
      return Err(error_message);
    }
  }

  let Some(expr) = attribute.expr.as_ref() else { unreachable!(); };

  Ok(())
}

fn fill_symbol_table_method(method: &Method, symbol_table: &mut SymbolTable) -> Result<(), String> {
  Ok(())
}

fn fill_symbol_table_expr(expr: &Expression, symbol_table: &mut SymbolTable, env_type: EnvType) -> Result<SymbolType, String> {
  symbol_table.enter_scope();
  let mut error_message = String::new();
  match expr {
    Expression::PartialAssign { .. } |
    Expression::PartialDispatch { .. } |
    Expression::PartialCastDispatch { .. } |
    Expression::PartialBinary { .. } => panic!("Unexpected intermediate expression: {:?}", expr),

    // No type check needed
    Expression::SelfTypeExpr { .. } |
    Expression::SelfExpr |
    Expression::StringExpr { .. } |
    Expression::IntExpr { .. } |
    Expression::BoolExpr { .. } => {}

    Expression::Assign { name, expr } => {
      fill_symbol_table_expr(expr, symbol_table, env_type)?;
      symbol_table.put(Symbol{name: name.get_name(), env_type, sym_type: SymbolType::Ident {}});
    }

    Expression::Plus { left, right } |
    Expression::Minus { left, right } |
    Expression::Multiply { left, right } |
    Expression::Divide { left, right } |
    Expression::LessThanOrEqual { left, right } |
    Expression::Equal { left, right } |
    Expression::LessThan { left, right } => {}

    Expression::Dispatch { calling_expr, cast_type, fn_name, param_list } => {}
    Expression::Conditional { .. } => {}
    Expression::Loop { .. } => {}
    Expression::Case { .. } => {}
    Expression::Block { .. } => {}
    Expression::Let { .. } => {}

    Expression::Negate { expr } |
    Expression::Not { expr } |
    Expression::IsVoid { expr } => {}

    Expression::IdentExpr { name } => {}

    Expression::New { type_name } => {
      if !symbol_table.lookup_symbol_by_env_type(type_name.get_name().as_str(), EnvType::Class) {
        error_message.push_str(format!("Type {type_name} not defined").as_str());
      }
    }
  }
  symbol_table.exit_scope();

  if error_message.is_empty() {
    Ok(())
  } else {
    Err(error_message)
  }
}
