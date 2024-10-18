use parser::get_ast;
use parser::model::class::Class;
use parser::model::program::Program;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs::File;

trait Init {
  type Node;
}

trait BlankInit: Init {
  
  fn new(name: String) -> Self::Node;
}

trait ParentInit: Init {
  
  fn new(name: String, parent: Option<ClassNode>) -> Self::Node;
}

trait WithChildrenInit: Init {
  fn new(name: String, parent: Option<ClassNode>, children: Vec<ClassNode>) -> Self::Node;
}

#[derive(Debug, PartialEq, Clone)]
struct ClassNode {
  name: Cow<'static, str>, 
  parent: Option<Box<ClassNode>>,
  children: Vec<ClassNode>,
}


/// Base object 
const OBJECT: ClassNode =  ClassNode {
  name: Cow::Owned("OBJECT".into()),
  parent: None,
  children: Vec::new(),
};

impl Init for ClassNode { type Node = ClassNode; }

impl ClassNode{
  fn new(name: String) -> ClassNode {
    ClassNode { name: Cow::from(name), parent: Some(Box::from(OBJECT.clone())), children: Vec::new()}
  }
}

/*impl ParentInit for ClassNode {

  fn new(name: String,parent: Option<Box<ClassNode>>) -> Self::Node {
    if parent.is_none() {
      ClassNode { name, parent: Some(Box::from(OBJECT)), children: Vec::new()}
    } else {
      ClassNode { name, parent, children: Vec::new() }
    }
  }
}
*/
/*impl WithChildrenInit for ClassNode {
  fn new(name: String,parent: Option<Box<ClassNode>>, children: Vec<ClassNode>) -> Self::Node {
   if parent.is_none() {
     ClassNode { name, parent: Some(Box::from(OBJECT)), children}
   } else {
     ClassNode { name, parent, children }
   }
  }
}
*/



fn gen_inheritance_graph(program: Program) -> HashMap<String, ClassNode> {
  let mut class_map: HashMap<String, ClassNode> = HashMap::new(); 
  let mut parent_set : HashSet<String> = HashSet::new();
  parent_set.insert("OBJECT".to_string());
  
  
  
  // First pass extract all class name and put it in the map
  let mut classes = program.classes;
  for Class{name: class_type, ..} in &classes {
    let class_name = class_type.get_name();
    let node =  ClassNode::new(class_name.clone());
    
    class_map.insert(class_name, node);
  }
  
  // Second pass, fill parents
  for Class{name: class_type, parent_type, ..} in &classes {
    let class_name = class_type.get_name();
    let Some(&ClassNode{ref mut parent, ..}) =  class_map.get(&class_name) else { unreachable!() };
    
    if let Some(class_parent) = parent_type {
      let parent_name = class_parent.get_name();
      let parent_node = ClassNode::new(parent_name);
      let _ = parent.insert(Box::new(parent_node));
    } else {
      let _ = parent.insert(Box::new(OBJECT));
    }
    
  }
  
  class_map
}

fn gen_graph(file:File) -> HashMap<String, ClassNode> {
  let program  = match get_ast(file) {
    Ok(pgm) => pgm,
    Err(e) => panic!("{e}"),
  };
  
  gen_inheritance_graph(program)
}


#[cfg(test)]
mod test {
  use crate::models::inheritance::gen_graph;
  use std::fs::File;

  #[test]
  fn test_gen_inheritance_graph() {
    let file = File::open("../test_resources/programs/arith.cl").expect("Couldn't open file");
    let graph = gen_graph(file);
    
    for (class, node) in graph {
      println!("{class} : {:#?}", node);
    }
  }
}
