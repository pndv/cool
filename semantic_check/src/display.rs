use crate::models::class_node::ClassNode;
use crate::models::expr::{AssignNode, BoolNode, CaseNode, ConditionalNode, DivNode, EqNode, ExprBlockNode, ExprNode, FalseNode, IdNode, IntNode, IsVoidNode, LetNode, LtEqNode, LtNode, MethodCallNode, MinusNode, MultiplyNode, NegateNode, NewNode, NotNode, PlusNode, SelfNode, SelfTypeNode, StringNode, TrueNode, WhileNode};
use crate::models::feature_node::{FeatureNode, FormalNode};
use crate::models::program_node::ProgramNode;
use crate::models::Node;
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
    let args = self.args.iter().map(|arg| format!("\t{}", arg.to_string())).collect::<Vec<String>>().join(",");
    let mut method_name = self.method_name.clone();
    if self.expr_cast_type.is_some() {
      method_name += format!("(@{})", self.expr_cast_type.clone().unwrap()).as_str();
    }
    write!(f, "[EXPR:METHOD_CALL] {}: [ARGS] {}\n\t[EXPR]:\n\t{}", self.method_name, args, self.expr.to_string())
  }
}
impl Display for ConditionalNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:CONDITIONAL]")
  }
}
impl Display for WhileNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:WHILE]")
  }
}
impl Display for ExprBlockNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:EXPR_BLOCK]")
  }
}
impl Display for LetNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:LET]")
  }
}
impl Display for CaseNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:CASE]")
  }
}
impl Display for NewNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:NEW]: {}", self.class)
  }
}
impl Display for IsVoidNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:ISVOID]")
  }
}
impl Display for PlusNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:PLUS]")
  }
}
impl Display for MinusNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:MINUS]")
  }
}
impl Display for MultiplyNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:MULTIPLY]")
  }
}
impl Display for DivNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:DIV]")
  }
}
impl Display for NegateNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:NEGATE]")
  }
}
impl Display for LtNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:LT]")
  }
}
impl Display for LtEqNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:LTEQ]")
  }
}
impl Display for EqNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:EQ]")
  }
}
impl Display for NotNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:NOT]:\n\t{}", self.expr.to_string())
  }
}
impl Display for IdNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:ID]: {}", self.name)
  }
}
impl Display for IntNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:INT]: {}", self.val)
  }
}
impl Display for StringNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:STRING]: {}", self.val)
  }
}
impl Display for BoolNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:BOOL]: {}", self.val)
  }
}
impl Display for TrueNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:TRUE]")
  }
}
impl Display for FalseNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:FALSE]")
  }
}
impl Display for SelfTypeNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:SELF_TYPE]")
  }
}
impl Display for SelfNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[EXPR:SELF]")
  }
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::Program { node } => write!(f, "{}", node.to_string()),
      Node::Class { node } => write!(f, "{}", node.to_string()),
      Node::Feature { node } => write!(f, "{}", node.to_string()),
      Node::Formal { node } => write!(f, "{}", node.to_string()),
      Node::Expr { node } => write!(f, "{}", node.to_string()),
    }
  }
}
