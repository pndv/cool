use crate::models::class_node::ClassNode;
use crate::models::expr::{AssignNode, BoolNode, CaseNode, ConditionalNode, DivNode, EqNode, ExprBlockNode, ExprNode, FalseNode, IdNode, IntNode, IsVoidNode, LetNode, LtEqNode, LtNode, MethodCallNode, MinusNode, MultiplyNode, NegateNode, NewNode, NotNode, PlusNode, SelfNode, SelfTypeNode, StringNode, TrueNode, WhileNode};
use crate::models::feature_node::{FeatureNode, FormalNode};
use crate::models::program_node::ProgramNode;
use std::fmt::{Display, Formatter, Pointer};

impl Display for ProgramNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let classes_str = self.class_map.values().map(|class| format!("\t{}", class.to_string())).collect::<Vec<String>>().join("\n");
    write!(f, "[PROGRAM]\n{}", classes_str)
  }
}

impl Display for ClassNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let parent_name = if self.get_parent_name().is_some() {
      format!(":{}", self.get_parent_name().unwrap())
    } else {
      String::from("")
    };
    let features_str = self.feature_map.iter().map(|(_, feature)| format!("\t{}", feature.to_string())).collect::<Vec<String>>().join("\n");
    write!(f, "[CLASS] {}{}\n{}", self.name, parent_name, features_str)
  }
}

impl Display for FeatureNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let expr_str = match &self.feature_expr {
      None => String::from(""),
      Some(e) => format!("\t{}", e.to_string())
    };

    match &self.param_type_map {
      None => write!(f, "[FEATURE][ATTRIBUTE] {}:{}\n{}", self.name, self.feature_type, expr_str),
      Some(params) => {
        let param_strs: Vec<String> = params.iter().map(|p| format!("{}:{}", p.name, p.formal_type)).collect();
        let param_str = param_strs.join(", ");
        write!(f, "[FEATURE][METHOD] {} ({}):{}\n{}", self.name, param_str, self.feature_type, expr_str)
      }
    }
  }
}

impl Display for FormalNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[IDENT] {}:{}", self.name.clone(), self.formal_type.clone())
  }
}

impl Display for ExprNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for AssignNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[ASSIGN] {}\n\t{}", self.id, self.expr.to_string())
  }
}

impl Display for MethodCallNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for ConditionalNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for WhileNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for ExprBlockNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for LetNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for CaseNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for NewNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for IsVoidNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for PlusNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for MinusNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for MultiplyNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for DivNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for NegateNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for LtNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for LtEqNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for EqNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for NotNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for IdNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for IntNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for StringNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for BoolNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for TrueNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for FalseNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for SelfTypeNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for SelfNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
