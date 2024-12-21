use crate::models::class::ClassNode;
use crate::models::symbols::{EnvType, Symbol, SymbolTable, SymbolType};

fn gen_sym_tab(class: ClassNode) -> Result<SymbolTable, String> {
    let mut table = SymbolTable::new();

    let class_symbol = Symbol {
        name: class.name.to_string(),
        env_type: EnvType::Class,
        sym_type: SymbolType::Type,
    };

    for feature in class.features {

    }

    unimplemented!()
}