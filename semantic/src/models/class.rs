
use std::fs::File;
use std::collections::{HashMap, HashSet};
use parser::get_ast;
use parser::model::class::{Class, BOOL_CLASS_NAME, INT_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME, STR_CLASS_NAME};
use std::borrow::Cow;
use parser::model::program::Program;
use std::fmt::{Display, Formatter};
use crate::models::features::FeatureNode;
use crate::models::Node;

const NO_INHERIT: [&str; 3] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME];
const PRIMITIVE_TYPES: [&str; 5] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME];


#[derive(Debug, PartialEq, Clone)]
struct ClassNode {
    name: Cow<'static, str>,
    parent: Cow<'static, str>,
    children: Vec<Cow<'static, str>>,
    features: Vec<dyn FeatureNode>
}

impl Node for ClassNode {}

impl ClassNode {
    fn add_child(&mut self, child: Cow<'static, str>) {
        self.children.push(child);
    }
}

impl From<Class> for ClassNode {
    fn from(value: Class) -> Self {
        let Class { name, parent_type, .. } = value;
        let class_name = name.get_name();
        let parent = parent_type.get_name();
        let children = Vec::new();

        ClassNode { name: Cow::from(class_name), parent: Cow::from(parent), children, features: Vec::new() }
    }
}

impl From<&Class> for ClassNode {
    fn from(value: &Class) -> Self {
        let Class { name, parent_type, .. } = value;
        let class_name = name.get_name();
        let parent = parent_type.get_name();
        let children = Vec::new();

        ClassNode { name: Cow::from(class_name), parent: Cow::from(parent), children, features: Vec::new() }
    }
}

impl Display for ClassNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let children: String = if self.children.len() > 0 {
      self.children.join(",")
    } else {
      " None".to_string()
    };
    write!(f, "[[ {} ]] Inherits from [[ {} ]] with children: [[ {} ]]", self.name, self.parent, children)
  }
}

const BASE_NODE_OBJECT: ClassNode = ClassNode {
  name: Cow::Borrowed(OBJECT_CLASS_NAME),
  parent: Cow::Borrowed(""),
  children: Vec::new(),
  features: Vec::new(),
};
const BASE_NODE_IO: ClassNode = ClassNode {
  name: Cow::Borrowed(IO_CLASS_NAME),
  parent: Cow::Borrowed(OBJECT_CLASS_NAME),
  children: Vec::new(),
  features: Vec::new(),
};

fn check_if_dag(node_map: &mut HashMap<String, ClassNode>, seen_nodes: &mut Vec<String>, start_class_name: &str) -> Result<Option<String>, String> {
  let gen_cycle_err = |nodes: Vec<String>| -> String {
    let chain = nodes.clone().join(" -> ");
    return format!("There is a cycle in the inheritance graph via {chain}");
  };

  if node_map.is_empty() {
    return Ok(None); // empty node_map => all nodes are seen
  }

  if seen_nodes.contains(&start_class_name.to_string()) {
    return Ok(Some(gen_cycle_err(seen_nodes.clone()))); // node is seen before; cycle in the graph
  }

  let node = match node_map.remove(start_class_name) {
    Some(n) => n,
    None if !node_map.contains_key(start_class_name) => {
      let chain = seen_nodes.clone().join(" -> ");
      return Ok(Some(format!("Class {start_class_name} is not declared in the chain {chain}")));
    }
    None => return Err(format!("Could not remove {start_class_name} from the node_map")),
  };

  let node_name = node.name;
  seen_nodes.push(node_name.into());

  let children = node.children;
  for child in children {
    let child_result = check_if_dag(node_map, seen_nodes, &*child)?;
    if child_result.is_some() { // seen a cycle, stop the loop, and return the result
      return Ok(child_result);
    }
  }

  seen_nodes.pop();
  Ok(None) // no cycle seen
}

fn gen_class_map<'a>(program: Program) -> HashMap<String, ClassNode> {
  let mut class_map: HashMap<String, ClassNode> = HashMap::new();
  let mut parent_set: HashSet<Cow<'a, str>> = HashSet::new();
  class_map.insert(BASE_NODE_OBJECT.name.to_string(), BASE_NODE_OBJECT.clone());
  class_map.insert(BASE_NODE_IO.name.to_string(), BASE_NODE_IO.clone());

  // First pass extracts all class names and parents and put it in the HashMap
  let classes = program.classes;
  for class in &classes {
    let node = ClassNode::from(class);

    let class_name = class.name.get_name();
    let line_num = class.line_num;
    let line_pos = class.line_pos;
    if PRIMITIVE_TYPES.contains(&class_name.as_ref()) {
      panic!("Error: {line_num}:{line_pos} attempt to inherit from sealed class via {class_name}")
    }

    let parent_name = class.parent_type.get_name();
    if NO_INHERIT.contains(&parent_name.as_ref()) {
      panic!("Error: {line_num}:{line_pos} attempt to inherit from sealed class via {parent_name}");
    }

    if class_name == parent_name {
      panic!("Error: {line_num}: {line_pos} class {class_name} attempted to inherit from itself");
    }

    parent_set.insert(node.parent.clone());
    class_map.insert(node.name.to_string(), node);
  }

  // In the second pass, link all children to parents
  for class in classes {
    let Some(class_node) = class_map.remove(&class.name.get_name()) else {
      unreachable!("Missing entry in class_map for {}", class.name.get_name())
    };
    let parent_name = &class_node.parent.to_string();
    let Some(mut parent) = class_map.remove(parent_name) else {
      unreachable!("Missing entry in class_map for parent {}", parent_name);
    };
    parent.add_child(Cow::from(class_node.name.clone()));
    class_map.insert(parent.name.to_string(), parent);
    class_map.insert(class.name.get_name(), class_node);
  }

  class_map
}

fn gen_graph<'a>(file: File) -> Result<HashMap<String, ClassNode>, String> {
  let program = match get_ast(file) {
    Ok(pgm) => pgm,
    Err(e) => panic!("{e}"),
  };

  let map = gen_class_map(program);

  match check_if_dag(&mut map.clone(), &mut Vec::new(), OBJECT_CLASS_NAME) {
    Ok(None) => Ok(map),
    Ok(Some(semantic_error)) => Err(semantic_error),
    Err(fatal_error) => panic!("{fatal_error}"),
  }
}

#[cfg(test)]
mod test {
  use parser::model::class::OBJECT_CLASS_NAME;
  use std::borrow::Cow;
  use std::collections::HashMap;
  use std::fs::File;
    use crate::models::class::{check_if_dag, gen_graph, ClassNode};

    #[test]
  fn test_gen_inheritance_graph() {
    let file = File::open("../test_resources/programs/arith.cl").expect("Couldn't open file");
    let graph_result = gen_graph(file);
    if graph_result.is_err() {
      panic!("Test failed: {:#?}", graph_result.err().unwrap());
    }
    assert!(graph_result.is_ok());
    let graph = graph_result.unwrap();

    for (class, _) in &graph {
      print!("{class} ");
    }

    for (class, node) in &graph {
      println!("{class} : {node}");
    }
  }

  #[test]
  fn test_cycle_positive() {
    let file = File::open("../test_resources/programs/arith.cl").expect("Couldn't open file");
    let graph_result = gen_graph(file);
    if graph_result.is_err() {
      panic!("Test failed: {:#?}", graph_result.err().unwrap());
    }
    assert!(graph_result.is_ok());
    let mut graph = graph_result.unwrap();

    let mut seen_nodes: Vec<String> = Vec::new();
    let cycle = check_if_dag(&mut graph, &mut seen_nodes, OBJECT_CLASS_NAME);
    if cycle.is_err() {
      println!("{:#?}", cycle.unwrap_err());
      assert!(false);
      return;
    }

    assert!(cycle.is_ok());
    assert!(cycle.unwrap().is_none());
  }

  #[test]
  fn test_cycle_negative() {
    // Hierarchy:
    // `Object` -> `A`
    // `A` -> `B`, `C`
    // `B` -> `C`
    // `C` -> `A`
    // `C` -> `D`

    let mut graph = HashMap::new();
    graph.insert(OBJECT_CLASS_NAME.to_string(), ClassNode { name: Cow::Borrowed(OBJECT_CLASS_NAME), parent: Cow::Borrowed(""), children: vec![Cow::Borrowed("A")], features: Vec::new() });
    graph.insert(String::from("A"), ClassNode { name: Cow::Borrowed("A"), parent: Cow::Borrowed(OBJECT_CLASS_NAME), children: vec![Cow::Borrowed("B"), Cow::Borrowed("C")], features: Vec::new() });
    graph.insert(String::from("B"), ClassNode { name: Cow::Borrowed("B"), parent: Cow::Borrowed("A"), children: vec![Cow::Borrowed("C")], features: Vec::new() });
    graph.insert(String::from("C"), ClassNode { name: Cow::Borrowed("C"), parent: Cow::Borrowed("B"), children: vec![Cow::Borrowed("A")], features: Vec::new() });
    graph.insert(String::from("D"), ClassNode { name: Cow::Borrowed("D"), parent: Cow::Borrowed("C"), children: Vec::new(), features: Vec::new() });
    let mut seen_nodes: Vec<String> = Vec::new();
    let cycle = check_if_dag(&mut graph, &mut seen_nodes, OBJECT_CLASS_NAME);
    if cycle.is_err() {
      println!("{:#?}", cycle.unwrap_err());
      assert!(false);
      return;
    }

    assert!(cycle.is_ok());
    let result = cycle.unwrap();
    assert!(result.is_some());
    let err = result.unwrap();
    println!("{err:?}");
    assert_eq!(err, "There is a cycle in the inheritance graph via Object -> A -> B -> C");
  }
}