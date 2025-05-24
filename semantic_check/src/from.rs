use crate::models::class_node::ClassNode;
use crate::models::expr::{AssignNode, BoolNode, CaseBranchNode, CaseNode, ConditionalNode, DivNode, EqNode, ExprBlockNode, ExprNode, IdNode, IntNode, IsVoidNode, LetInitNode, LetNode, LtEqNode, LtNode, MethodCallNode, MinusNode, MultiplyNode, NegateNode, NewNode, NotNode, PlusNode, SelfNode, SelfTypeNode, StringNode, WhileNode};
use crate::models::feature_node::{FeatureNode, FormalNode};
use parser::model::class::ParseClass;
use parser::model::expressions::Expression::{Assign, Block, BoolExpr, Case, Conditional, Dispatch, Divide, Equal, IdentExpr, IntExpr, IsVoid, LessThan, LessThanOrEqual, Let, Loop, Minus, Multiply, Negate, New, Not, Plus, SelfExpr, StringExpr};
use parser::model::expressions::{CaseBranch, Expression, LetInit};
use parser::model::feature::{Attribute, Method, ParseFeature};
use parser::model::formal::Formal;
use std::collections::HashMap;

impl From<&ParseClass> for ClassNode {
  fn from(value: &ParseClass) -> Self {
    // dbg!("Call ClassNode::from with: {}", value);
    match value {
      ParseClass { name, parent_type, features, .. } => {
        let mut nodes_map: HashMap<String, FeatureNode> = Default::default();
        let feature_nodes = match features {
          None => nodes_map,
          Some(parse_features) => parse_features.iter().map(|parse_feature| {
            let mut feature_node = FeatureNode::from(parse_feature);
            (feature_node.name.clone(), feature_node)
          }).collect()
        };

        Self { name: name.get_name().clone(), parent: None, children: Some(vec![]), feature_map: feature_nodes }
      }
    }
  }
}

impl From<Expression> for ExprNode {
  fn from(value: Expression) -> Self {
    ExprNode::from(&value)
  }
}

impl From<LetInit> for LetInitNode {
  fn from(value: LetInit) -> Self {
    LetInitNode::from(&value) 
  }
}

impl From<&LetInit> for LetInitNode {
  fn from(value: &LetInit) -> Self {
    let LetInit { id, id_type, expr } = value;
    LetInitNode {
      id: id.get_name(),
      id_type: id_type.get_name(),
      expr: expr.as_ref().map(|e| Box::from(ExprNode::from(e))),
    }
  }
}

impl From<CaseBranch> for CaseBranchNode {
  fn from(value: CaseBranch) -> Self {
    CaseBranchNode::from(&value)
  }
}

impl From<&CaseBranch> for CaseBranchNode {
  fn from(value: &CaseBranch) -> Self {
    let CaseBranch { id, id_type, expr } = value;
    CaseBranchNode {
      id: id.get_name(),
      id_type: id_type.get_name(),
      expr: Box::from(ExprNode::from(expr)),
    }
  }
}

impl From<&Box<Expression>> for ExprNode {
  fn from(value: &Box<Expression>) -> Self {
    ExprNode::from(value.as_ref())
  }
}

impl From<&Expression> for ExprNode {
  fn from(expr: &Expression) -> Self {
    match expr {
      Expression::PartialAssign { .. } | Expression::PartialDispatch { .. } | Expression::PartialCastDispatch { .. } | Expression::PartialBinary { .. } => { 
        panic!("Should not have partial expressions: {expr}") 
      },

      Expression::SelfTypeExpr {..} => ExprNode::SelfTypeNode { node: SelfTypeNode {} },
      SelfExpr => ExprNode::SelfNode { node: SelfNode {} },

      Assign { name, expr } => ExprNode::Assign {
        node: AssignNode {
          id: name.get_name(),
          expr: Box::from(ExprNode::from(expr)),
        }
      },
      Dispatch { calling_expr, cast_type, fn_name, param_list } => ExprNode::MethodCall {
        node: MethodCallNode {
          expr: Box::from(ExprNode::from(calling_expr)), // If missing, parser adds `self`
          expr_cast_type: cast_type.clone().map(|t| t.get_name()),
          method_name: fn_name.get_name(),
          args: param_list.into_iter().map(|expr| Box::from(ExprNode::from(expr))).collect(),
        }
      },

      Conditional { predicate, then_expr, else_expr } => ExprNode::Conditional {
        node: ConditionalNode {
          condition: Box::from(ExprNode::from(predicate)),
          true_expr: Box::from(ExprNode::from(then_expr)),
          false_expr: Box::from(ExprNode::from(else_expr)),
        }
      },
      Loop { predicate, body } => ExprNode::While {
        node: WhileNode {
          condition: Box::from(ExprNode::from(predicate)),
          body: Box::from(ExprNode::from(body)),
        }
      },
      Case { switch_expression, branches } => ExprNode::Case {
        node: CaseNode {
          expr: Box::from(ExprNode::from(switch_expression)),
          branches: branches.into_iter().map(|branch| branch.into()).collect(),
        }
      },
      Block { expr_list } => ExprNode::ExprBlock {
        node: ExprBlockNode {
          expressions: expr_list.into_iter().map(|expr| Box::from(ExprNode::from(expr))).collect(),
        }
      },
      Let { let_init, in_expr } => ExprNode::Let {
        node: LetNode {
          let_init: let_init.into_iter().map(|let_init| let_init.into()).collect(),
          in_expr: Box::from(ExprNode::from(in_expr)),
        }
      },

      Plus { left, right } => ExprNode::Plus { node: PlusNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      Minus { left, right } => ExprNode::Minus { node: MinusNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      Multiply { left, right } => ExprNode::Multiply { node: MultiplyNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      Divide { left, right } => ExprNode::Div { node: DivNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      LessThan { left, right } => ExprNode::Lt { node: LtNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      Equal { left, right } => ExprNode::Eq { node: EqNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },
      LessThanOrEqual { left, right } => ExprNode::LtEq { node: LtEqNode { left: Box::from(ExprNode::from(left)), right: Box::from(ExprNode::from(right)) } },

      Negate { expr } => ExprNode::Negate { node: NegateNode { expr: Box::from(ExprNode::from(expr)) } },
      Not { expr } => ExprNode::Not { node: NotNode { expr: Box::from(ExprNode::from(expr)) } },

      IdentExpr { name } => ExprNode::Id { node: IdNode { name: name.get_name() } },

      IntExpr { value, .. } => ExprNode::Int { node: IntNode { val: value.clone() } },
      BoolExpr { value, .. } => ExprNode::Bool { node: BoolNode { val: value.clone() } },
      StringExpr { value, .. } => ExprNode::String { node: StringNode { val: value.clone() } },

      New { type_name } => ExprNode::New { node: NewNode { class: type_name.get_name() } },
      IsVoid { expr } => ExprNode::IsVoid { node: IsVoidNode { expr: Box::from(ExprNode::from(expr)) } },
    }
  }
}

impl From<&ParseFeature> for FeatureNode {
  fn from(value: &ParseFeature) -> Self {
    match value {
      ParseFeature::Attribute { attribute: Attribute { name, return_type, expr } } => {
        Self { name: name.get_name(), 
          param_type_map: None, 
          feature_type: return_type.get_name(), 
          feature_expr: expr.as_ref().map(|e| ExprNode::from(e)) 
        }
      }
      ParseFeature::Method { method: Method { name, formals, return_type, expr } } => {
        let params = match formals {
          None => vec![],
          Some(params) => params.iter().map(|param| {
            FormalNode::from(param)
          }).collect()
        };
        
        Self { 
          name: name.get_name(), 
          param_type_map: Some(params), 
          feature_type: return_type.get_name(), 
          feature_expr:  Some(ExprNode::from(expr))
        }
      }
    }
  }
}

impl From<&Formal> for FormalNode {
  fn from(value: &Formal) -> Self {
    match value {
      Formal { formal_name, formal_type } => Self { name: formal_name.get_name(), formal_type: formal_type.get_name() }
    }
  }
}
