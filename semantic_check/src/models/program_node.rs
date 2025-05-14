use crate::models;
use crate::models::class_node::ClassNode;
use crate::models::Node;
use crate::symbol_table::SymbolTable;
use parser::model::class::OBJECT_CLASS_NAME;
use parser::model::program::ParseProgram;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct ProgramNode {
  class_map: HashMap<String, ClassNode>,
}

impl Display for ProgramNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let classes_str = self.class_map.values().map(|class| format!("\t{}", class.to_string())).collect::<Vec<String>>().join("\n");
    write!(f, "[PROGRAM]\n{}", classes_str)
  }
}

pub fn init_global_scope(program: ParseProgram, symbol_table: &mut SymbolTable) -> Result<(), String> {
  insert_base_object(symbol_table);
  insert_program_classes(program, symbol_table);
  let check_dag = check_class_graph_dag(symbol_table);
  check_dag
}

pub fn insert_base_object(symbol_table: &mut SymbolTable) {
  let base_object = ClassNode::get_object_node();
  let base_object_name: String = base_object.read().unwrap().name().to_string();

  // Put `object` and its immediate children in the symbol table (IO, INT, BOOL, STRING)
  match symbol_table.insert_symbol(base_object_name.as_ref(), base_object.clone()) {
    Ok(_) => {}
    Err(error) => panic!("{}", error)
  }
  let base_object_ref = base_object.write().unwrap();
  let Node::Class { node: base_object_node } = base_object_ref.deref() else { panic!("Node is not a ClassNode") };
  let Some(base_children) = base_object_node.get_children() else { unreachable!() };
  for child in base_children {
    match symbol_table.insert_symbol(child.read().unwrap().name(), child.clone()) {
      Ok(_) => {}
      Err(error) => panic!("{}", error)
    }
  }
}

// Populate symbol table
pub fn insert_program_classes<'a>(program: ParseProgram, symbol_table: &mut SymbolTable) {
  // First pass extracts all class names and parents and put it in the HashMap, populate the symbol table
  let classes = &program.classes;
  for class in classes {
    let class_name = class.name.get_name();
    let parent_name = class.parent_type.get_name();
    let line_num = class.line_num;
    let line_pos = class.line_pos;

    if class_name == parent_name {
      panic!("Error: {line_num}: {line_pos} class {class_name} attempted to inherit from itself");
    }

    if models::class_node::NO_INHERIT.contains(&parent_name.as_ref()) {
      panic!("Error: {line_num}:{line_pos} attempt to inherit from sealed class via {parent_name}");
    }

    let node = Node::Class { node: ClassNode::from(class) };
    let node_name = node.name().to_string();
    match symbol_table.insert_symbol(node_name.as_str(), Arc::new(RwLock::new(node))) {
      Ok(_) => {}
      Err(error) => panic!("{}", error)
    }
  }

  // In the second pass, link all children to parents
  for class in classes {
    let parse_class_name = class.name.get_name();
    let parse_parent_name = class.parent_type.get_name();
    let Some(class_node) = symbol_table.lookup_symbol(parse_class_name.as_ref()) else { panic!("Missing entry in symbol_table for {}", parse_class_name) };
    let Some(parent_node) = symbol_table.lookup_symbol(parse_parent_name.as_ref()) else { panic!("Missing entry in symbol_table for {}", parse_parent_name) };

    let mut class_node_ref = class_node.write().unwrap();
    let c = class_node_ref.deref_mut();
    let Node::Class { node: cls } = c else { panic!("Node is not a ClassNode") };
    cls.set_parent(parent_node.clone());

    let pn = parent_node.write();
    let mut parent_node_ref = pn.unwrap();
    let p = parent_node_ref.deref_mut();
    let Node::Class { node: prn } = p else { panic!("Node is not a ClassNode") };

    prn.add_child(&class_node);
  }
}

pub fn check_class_graph_dag(symbol_table: &mut SymbolTable) -> Result<(), String> {
  let global_scope = symbol_table.get_global_scope();
  let mut node_map: HashMap<String, ClassNode> = HashMap::new();
  for (class_name, node) in global_scope {
    let node_ref = node.read().unwrap();
    let Node::Class { node: class_ref } = node_ref.deref() else { panic!("Node is not a ClassNode") };
    node_map.insert(class_name.clone(), class_ref.clone());
  }

  let mut seen_nodes: Vec<String> = Vec::new();
  let start_class_name = OBJECT_CLASS_NAME;

  check_graph_helper(&mut node_map, &mut seen_nodes, start_class_name)
}

fn check_graph_helper(node_map: &mut HashMap<String, ClassNode>, seen_nodes: &mut Vec<String>, start_class_name: &str) -> Result<(), String> {
  let gen_cycle_err = |nodes: Vec<String>| -> String {
    let chain = nodes.clone().join(" -> ");
    return format!("There is a cycle in the inheritance graph via {chain}");
  };

  if node_map.is_empty() {
    return Ok(()); // empty node_map => all nodes are seen
  }

  if seen_nodes.contains(&start_class_name.to_string()) {
    return Err(gen_cycle_err(seen_nodes.clone())); // node is seen before; cycle in the graph
  }

  let node = match node_map.remove(start_class_name) {
    Some(n) => n,
    None if !node_map.contains_key(start_class_name) => {
      let chain = seen_nodes.clone().join(" -> ");
      return Err(format!("Class {start_class_name} is not declared in the chain {chain}"));
    }
    None => return Err(format!("Could not remove {start_class_name} from the node_map")),
  };

  let node_name = node.name.clone();
  seen_nodes.push(node_name.into());

  let children = node.get_children().unwrap_or_else(|| vec![]);

  for child in children {
    let child_ref = child.read().unwrap();
    let child_name = child_ref.name();
    let child_result = check_graph_helper(node_map, seen_nodes, child_name);
    if child_result.is_err() { // seen a cycle, stop the loop, and return the result
      return Err(child_result.err().unwrap());
    }
  }

  seen_nodes.pop();
  Ok(()) // no cycle seen
}
