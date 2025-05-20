use crate::models::feature_node::{FeatureNode, FormalNode};
use crate::models::Node;
use parser::model::class::{BOOL_CLASS_NAME, INT_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME, STR_CLASS_NAME};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

static NODE_OBJECT: OnceLock<Arc<RwLock<ClassNode>>> = OnceLock::new();
static NODE_IO: OnceLock<Arc<RwLock<ClassNode>>> = OnceLock::new();
static NODE_STRING: OnceLock<Arc<RwLock<ClassNode>>> = OnceLock::new();
static NODE_INT: OnceLock<Arc<RwLock<ClassNode>>> = OnceLock::new();
static NODE_BOOL: OnceLock<Arc<RwLock<ClassNode>>> = OnceLock::new();
pub(crate) static OBJECT: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();
pub(crate) static IO: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();
pub(crate) static STRING: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();
pub(crate) static INT: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();
pub(crate) static BOOL: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();

pub const NO_INHERIT: [&str; 3] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME];
pub const PRIMITIVE_TYPES: [&str; 5] = [INT_CLASS_NAME, STR_CLASS_NAME, BOOL_CLASS_NAME, IO_CLASS_NAME, OBJECT_CLASS_NAME];

use crate::symbol_table::SymbolTable;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct ClassNode {
  pub(crate) name: String,
  pub(crate) parent: Option<Arc<RwLock<Node>>>,
  pub(crate) children: Option<Vec<Arc<RwLock<Node>>>>,
  pub(crate) feature_map: HashMap<String, FeatureNode>,
}
impl ClassNode {
  pub fn get_object_node<'a>() -> &'a Arc<RwLock<Node>> {
    OBJECT.get_or_init(|| {
      let obj = Node::Class {
        node: ClassNode {
          name: String::from(OBJECT_CLASS_NAME),
          parent: None,
          children: Some(Vec::new()),
          feature_map: HashMap::new(),
        }
      };

      let obj_node = Arc::new(RwLock::new(obj));
      IO.get_or_init(|| {
        let io = Node::Class {
          node: ClassNode {
            name: String::from(IO_CLASS_NAME),
            parent: Some(obj_node.clone()),
            children: Some(Vec::new()),
            feature_map: HashMap::new(),
          }
        };

        Arc::new(RwLock::new(io))
      });

      STRING.get_or_init(|| {
        let string = Node::Class {
          node: ClassNode {
            name: String::from(STR_CLASS_NAME),
            parent: Some(obj_node.clone()),
            children: Some(Vec::new()),
            feature_map: HashMap::new(),
          }
        };

        Arc::new(RwLock::new(string))
      });

      INT.get_or_init(|| {
        let string = Node::Class {
          node: ClassNode {
            name: String::from(INT_CLASS_NAME),
            parent: Some(obj_node.clone()),
            children: Some(Vec::new()),
            feature_map: HashMap::new(),
          }
        };

        Arc::new(RwLock::new(string))
      });

      BOOL.get_or_init(|| {
        let string = Node::Class {
          node: ClassNode {
            name: String::from(BOOL_CLASS_NAME),
            parent: Some(obj_node.clone()),
            children: Some(Vec::new()),
            feature_map: HashMap::new(),
          }
        };

        Arc::new(RwLock::new(string))
      });

      let obj_node_ref_clone = obj_node.clone();
      let mut obj_node_unwrap = obj_node_ref_clone.write().unwrap();
      let obj_node_deref = obj_node_unwrap.deref_mut();
      let Node::Class { node: obj_class_node } = obj_node_deref else { panic!("Not a class node") };
      obj_class_node.children = Some(vec![IO.get().unwrap().clone(), STRING.get().unwrap().clone(), INT.get().unwrap().clone(), BOOL.get().unwrap().clone()]);

      obj_node
    })
  }
  
  pub(crate) fn get_children(&self) -> Option<Vec<Arc<RwLock<Node>>>> {
    self.children.clone()
  }

  pub(crate) fn add_child(&mut self, child: &Arc<RwLock<Node>>) {
    if let Some(ref mut children) = self.children {
      children.push(child.clone());
    } else {
      panic!("Inheritance not allowed for class {}", self.name);
    }
  }

  pub(crate) fn set_parent(&mut self, parent_node: Arc<RwLock<Node>>) {
    match &mut (self.parent) {
      None => self.parent = Some(parent_node.clone()),
      Some(parent) => {
        let parent_node_ref = parent.read().unwrap();

        let Node::Class { node: parent_class } = parent_node_ref.deref() else { panic!("Not a class node") };
        let parent_name = parent_class.name.clone();
        if parent_name == self.name {
          panic!("Class {} can't inherit itself", self.name)
        }

        let parent_node_ref = parent_node.read().unwrap();
        let Node::Class { node: input_parent_node } = parent_node_ref.deref() else { panic!("Not a class node") };

        if parent_name != input_parent_node.name {
          println!("Class {} already has another parent set {}", self.name, parent_name);
        }

        //NO-OP, parent is already set and 
      }
    }
  }

  pub(crate) fn get_parent(&self) -> Option<Arc<RwLock<Node>>> {
    self.parent.clone()
  }

  pub(crate) fn get_parent_name(&self) -> Option<String> {
    if let Some(ref node) = self.parent {
      let parent_node_ref = node.read().unwrap();
      let Node::Class { node: parent } = parent_node_ref.deref() else { panic!("Not a class node") };
      Some(parent.name.clone())
    } else {
      None
    }
  }
  
  pub(crate) fn check_features(&self, symbol_table: &mut SymbolTable) -> Result<(), String> {
    let mut errors: Vec<String> = Vec::new();
    
    // Populate class symbols
    symbol_table.enter_scope();
    
    for (feature_name, feature_node) in &self.feature_map {
      // type check features
      let FeatureNode { name, param_type_map, feature_type, feature_expr } = feature_node.clone();
      if symbol_table.lookup_symbol(feature_type.as_str()).is_none() {
        errors.push(format!("Feature {} has unknown type {}", feature_name, feature_type));
        continue;
      }
      
      if let Some(formal_nodes) = param_type_map {
        for formal_node in formal_nodes {
          let FormalNode  {name, formal_type, ..} = formal_node.clone();
          if symbol_table.lookup_symbol(formal_type.as_str()).is_none() {
            errors.push(format!("Formal {} has unknown type {}", name, formal_type));
          }       
        }        
      }
      
      if errors.len() > 0 {
        continue;    
      }
      
      // type check complete, insert this feature into the symbol table
      match symbol_table.insert_symbol(feature_name.clone().as_str(), 
                                       Arc::new(RwLock::new(Node::Feature { node: feature_node.clone() }))) {
        Ok(_) => (),
        Err(e) => errors.push(e)
      }
    }
    
    // Now iterate over expressions in features
    
    
    Ok(())
  }
}

