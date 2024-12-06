use crate::models::symbols::{EnvType, Symbol, SymbolTable, SymbolType};
use parser::model::class::ParseClass;
use parser::model::feature::ParseFeature;
use parser::model::program::ParseProgram;

fn gen_symbol_table(program: ParseProgram) -> Result<SymbolTable, String> {
  let mut symbol_table = SymbolTable::new();
  

  for class in program.classes {
    fill_symbol_table_class(&class, &mut symbol_table)?;
  }

  Ok(symbol_table)
}


fn fill_symbol_table_class(class: &ParseClass, symbol_table: &mut SymbolTable) -> Result<(), String> {
  symbol_table.enter_scope();

  symbol_table.put(Symbol {
    name: class.name.to_string(),
    env: EnvType::Class,
    sym_type: SymbolType::Type,
  });

  
  
  if class.features.is_none() {
    return Ok(());
  }
  
  for feature in class.features.as_ref().unwrap() {
    fill_symbol_table_feature(feature, symbol_table)?
  }
  

  

  symbol_table.exit_scope();
  Ok(())
}

fn fill_symbol_table_feature(feature: &ParseFeature, symbol_table: &mut SymbolTable) -> Result<(), String> {
  match feature {
      ParseFeature::Attribute{ attribute } => {
            symbol_table.put(Symbol {
                name: attribute.name.0.to_string(),
                env: EnvType::Class,
                sym_type: SymbolType::Ident,
            });
            Ok(())
        }
      ParseFeature::Method{method} => {
            symbol_table.enter_scope();
            symbol_table.put(Symbol {
                name: method.name.0.to_string(),
                env: EnvType::Class,
                sym_type: SymbolType::Ident,
            });
            Ok(())
        }
    }
}
