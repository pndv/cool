use crate::models::class_node::ClassNode;
use crate::models::class_node::OBJECT;
use crate::models::Node;
use crate::symbol_table::SymbolTable;
use models::program_node;
use parser::model::program::ParseProgram;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

pub(crate) mod models;
pub(crate) mod symbol_table;

fn start_semantic_check(program: ParseProgram) {
  let mut symbol_table: SymbolTable = Default::default();
  symbol_table.enter_scope();
  
  // populate classes and their respective methods and attributes
  match program_node::init_global_scope(program, &mut symbol_table) {
    Ok(_) => {},
    Err(error ) => panic!("Cyclic inheritance: {}", error)
  }
  
  // iterate over class's features in its scope and decorate it with its type 
  let global_scope = symbol_table.get_global_scope().clone();
  let mut errors: Vec<String> = Default::default();
  for node in global_scope.values() {
    let class_node = node.read().unwrap();
    let Node::Class { node: class } = class_node.deref() else { unreachable!() };
    match class.check_features(&mut symbol_table) {
      Ok(_) => {},
      Err(error) => errors.push(error)
    }
  }
  

}

#[cfg(test)]
mod test {
  use super::*;
  use crate::models::class_node::ClassNode;
  use crate::models::program_node::{check_class_graph_dag, init_global_scope, insert_base_object};
  use crate::symbol_table::SymbolTable;
  use parser::get_ast;
  use parser::model::program::ParseProgram;
  use std::fs::File;

  #[test]
  fn test_semantic_check() {
    let file = File::open("../test_resources/programs/graph.cl").expect("Couldn't open file");
    let parse_program: ParseProgram = get_ast(file).expect("Couldn't parse file");
    start_semantic_check(parse_program);
  }

  #[test]
  fn test_cycle_positive() {
    let file = File::open("../test_resources/programs/graph.cl").expect("Couldn't open file");
    let parse_program: ParseProgram = get_ast(file).expect("Couldn't parse file");
    let mut symbol_table: SymbolTable = Default::default();
    symbol_table.enter_scope();
    
    let init_result = init_global_scope(parse_program, &mut symbol_table);
    assert!(init_result.is_ok());
    
    let global_scope = symbol_table.get_global_scope();
    assert_eq!(global_scope.len(), 15);
    
    let class_names: Vec<String> = global_scope.keys().cloned().collect();
    println!("Global scope size: {}", global_scope.len());
    println!("Classes: {}", class_names.join(", "));
    for node in global_scope.values() {
      let class = node.read().unwrap();
      println!("Class: {}", class.to_string());
    }
  }

  fn gen_cyclic_graph(symbol_table: &mut SymbolTable) {
    // Hierarchy:
    // `Object` -> `A`
    // `A` -> `B`, `C`
    // `B` -> `C`
    // `C` -> `A`
    // `C` -> `D`

    insert_base_object(symbol_table);

    let Some(obj) = OBJECT.get() else { panic!("OBJECT not initialised") };

    let a = Arc::new(RwLock::new(Node::Class {
      node: ClassNode {
        name: String::from("A"),
        parent: Some(obj.clone()),
        children: Some(vec![]),
        feature_map: Default::default(),
      }
    }));
    symbol_table.insert_symbol("A", a.clone()).expect("failed to insert");

    let b = Arc::new(RwLock::new(Node::Class {
      node: ClassNode {
        name: String::from("B"),
        parent: Some(a.clone()),
        children: Some(vec![]),
        feature_map: Default::default(),
      }
    }));
    symbol_table.insert_symbol("B", b.clone()).expect("failed to insert");

    let c = Arc::new(RwLock::new(Node::Class {
      node: ClassNode {
        name: String::from("C"),
        parent: Some(a.clone()),
        children: Some(vec![a.clone()]),
        feature_map: Default::default(),
      }
    }));
    symbol_table.insert_symbol("C", c.clone()).expect("failed to insert");

    let d = Arc::new(RwLock::new(Node::Class {
      node: ClassNode {
        name: String::from("D"),
        parent: Some(c.clone()),
        children: Some(vec![]),
        feature_map: Default::default(),
      }
    }));
    symbol_table.insert_symbol("D", d.clone()).expect("failed to insert");

    let mut a_node_ref = a.write().unwrap();
    let Node::Class { node: a_node } = a_node_ref.deref_mut() else { unreachable!() };
    a_node.add_child(&b);
    a_node.add_child(&c);

    let mut b_node_ref = b.write().unwrap();
    let Node::Class { node: b_node } = b_node_ref.deref_mut() else { unreachable!() };
    b_node.add_child(&c);

    let mut obj_node_ref = obj.write().unwrap();
    let Node::Class { node: obj_node } = obj_node_ref.deref_mut() else { unreachable!() };
    obj_node.add_child(&a);
  }

  #[test]
  fn test_cycle_negative() {
    let mut symbol_table: SymbolTable = Default::default();
    symbol_table.enter_scope();
    gen_cyclic_graph(&mut symbol_table);
    // Hierarchy:
    // `Object` -> `A`
    // `A` -> `B`, `C`
    // `B` -> `C`
    // `C` -> `A`
    // `C` -> `D`

    let cycle = check_class_graph_dag(&mut symbol_table);
    assert!(cycle.is_err());

    let result = cycle.err();
    assert!(result.is_some());

    let err = result.unwrap();
    println!("{err:?}");
    assert_eq!(err, "There is a cycle in the inheritance graph via Object -> A -> B -> C");
  }
}
