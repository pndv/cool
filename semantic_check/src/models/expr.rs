use parser::model::expressions::{Expression, LetInit};
use std::fmt::Display;
use Expression::{Assign, Block, BoolExpr, Case, Conditional, Dispatch, Divide, Equal, IdentExpr, IntExpr, IsVoid, LessThan, LessThanOrEqual, Let, Loop, Minus, Multiply, Negate, New, Not, Plus, SelfExpr, StringExpr};

#[derive(Debug, Clone)]
pub(crate) enum ExprNode {
  Assign { node: AssignNode },
  MethodCall { node: MethodCallNode },
  Conditional { node: ConditionalNode },
  While { node: WhileNode },
  ExprBlock { node: ExprBlockNode },
  Let { node: LetNode },
  Case { node: CaseNode },
  New { node: NewNode },
  IsVoid { node: IsVoidNode },
  Plus { node: PlusNode },
  Minus { node: MinusNode },
  Multiply { node: MultiplyNode },
  Div { node: DivNode },
  Negate { node: NegateNode },
  Lt { node: LtNode },
  LtEq { node: LtEqNode },
  Eq { node: EqNode },
  Not { node: NotNode },
  Id { node: IdNode },
  Int { node: IntNode },
  String { node: StringNode },
  Bool { node: BoolNode },
  True { node: TrueNode },
  False { node: FalseNode },
  SelfTypeNode { node: SelfTypeNode },
  SelfNode { node: SelfNode },
}

#[derive(Debug, Clone)]
pub(crate) struct AssignNode {
  pub(crate) id: String,
  pub(crate) expr: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct MethodCallNode {
  expr: ExprNode,
  expr_cast_type: Option<String>, // Can only cast to types in its parent hierarchy
  method_name: String,
  args: Vec<ExprNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct ConditionalNode {
  condition: ExprNode,
  true_expr: ExprNode,
  false_expr: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct WhileNode {
  condition: ExprNode,
  body: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct ExprBlockNode {
  expressions: Vec<ExprNode>,
}
#[derive(Debug, Clone)]
pub(crate) struct LetInitNode {
  id: String,
  id_type: String,
  expr: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct LetNode {
  let_init: Vec<LetInitNode>,
  in_expr: ExprNode,
}
#[derive(Debug, Clone)]
pub(crate) struct CaseBranchNode {
  id: String,
  id_type: String,
  expr: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct CaseNode {
  expr: ExprNode,
  branches: Vec<CaseBranchNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct NewNode {
  class: String,
}

#[derive(Debug, Clone)]
pub(crate) struct IsVoidNode {
  expr: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct PlusNode {
  left: ExprNode,
  right: ExprNode,
}

#[derive(Debug, Clone)]
pub(crate) struct MinusNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct MultiplyNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct DivNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct NegateNode { expr: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct LtNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct LtEqNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct EqNode { left: ExprNode, right: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct NotNode { expr: ExprNode }

#[derive(Debug, Clone)]
pub(crate) struct IdNode { name: String }
#[derive(Debug, Clone)]
pub(crate) struct IntNode { val: i32 }
#[derive(Debug, Clone)]
pub(crate) struct StringNode {
  val: String,
}
#[derive(Debug, Clone)]
pub(crate) struct BoolNode {
  val: bool,
}
#[derive(Debug, Clone)]
pub(crate) struct SelfTypeNode {}
#[derive(Debug, Clone)]
pub(crate) struct SelfNode {}
#[derive(Debug, Clone)]
pub(crate) struct TrueNode {}
#[derive(Debug, Clone)]
pub(crate) struct FalseNode {}

impl From<Expression> for ExprNode {
  fn from(value: Expression) -> Self {
    ExprNode::from(&value)
  }
}
impl From<LetInit> for LetInitNode {
  fn from(value: LetInit) -> Self {
    let LetInit { id, id_type, expr } = value;
    LetInitNode {
      id: id.get_name(),
      id_type: id_type.get_name(),
      expr: expr.into(),
    }
  }
}
impl From<parser::model::expressions::CaseBranch> for CaseBranchNode {
  fn from(value: parser::model::expressions::CaseBranch) -> Self {
    let parser::model::expressions::CaseBranch { id, id_type, expr } = value;
    CaseBranchNode {
      id: id.get_name(),
      id_type: id_type.get_name(),
      expr: expr.into(),
    }
  }
}

impl From<&Expression> for ExprNode {
  fn from(expr: &Expression) -> Self {
    match expr {
      Expression::PartialAssign { .. } | Expression::PartialDispatch { .. } | Expression::PartialCastDispatch { .. } | Expression::PartialBinary { .. } => panic!("Should not have partial expressions"),

      Expression::SelfTypeExpr => ExprNode::SelfTypeNode { node: SelfTypeNode {} },
      SelfExpr => ExprNode::SelfNode { node: SelfNode {} },

      Assign { name, expr } => ExprNode::Assign {
        node: AssignNode {
          id: name.get_name(),
          expr: expr.into(),
        }
      },
      Dispatch { calling_expr, cast_type, fn_name, param_list } => ExprNode::MethodCall {
        node: MethodCallNode {
          expr: calling_expr.into(), // If missing, parser adds `self`
          expr_cast_type: cast_type.clone().map(|t| t.get_name()),
          method_name: fn_name.get_name(),
          args: param_list.into_iter().map(|expr| expr.into()).collect(),
        }
      },

      Conditional { predicate, then_expr, else_expr } => ExprNode::Conditional {
        node: ConditionalNode {
          condition: predicate.into(),
          true_expr: then_expr.into(),
          false_expr: else_expr.into(),
        }
      },
      Loop { predicate, body } => ExprNode::While {
        node: WhileNode {
          condition: predicate.into(),
          body: body.into(),
        }
      },
      Case { switch_expression, branches } => ExprNode::Case {
        node: CaseNode {
          expr: switch_expression.into(),
          branches: branches.into_iter().map(|branch| branch.into()).collect(),
        }
      },
      Block { expr_list } => ExprNode::ExprBlock {
        node: ExprBlockNode {
          expressions: expr_list.into_iter().map(|expr| expr.into()).collect(),
        }
      },
      Let { let_init, in_expr } => ExprNode::Let {
        node: LetNode {
          let_init: let_init.into_iter().map(|let_init| let_init.into()).collect(),
          in_expr: in_expr.into(),
        }
      },

      Plus { left, right } => ExprNode::Plus { node: PlusNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      Minus { left, right } => ExprNode::Minus { node: MinusNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      Multiply { left, right } => ExprNode::Multiply { node: MultiplyNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      Divide { left, right } => ExprNode::Div { node: DivNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      LessThan { left, right } => ExprNode::Lt { node: LtNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      Equal { left, right } => ExprNode::Eq { node: EqNode { left: ExprNode::from(left), right: ExprNode::from(right) } },
      LessThanOrEqual { left, right } => ExprNode::LtEq { node: LtEqNode { left: ExprNode::from(left), right: ExprNode::from(right) } },

      Negate { expr } => ExprNode::Negate { node: NegateNode { expr: ExprNode::from(expr) } },
      Not { expr } => ExprNode::Not { node: NotNode { expr: ExprNode::from(expr) } },

      IdentExpr { name } => ExprNode::Id { node: IdNode { name: name.get_name() } },

      IntExpr { value, .. } => ExprNode::Int { node: IntNode { val: value.clone() } },
      BoolExpr { value, .. } => ExprNode::Bool { node: BoolNode { val: value.clone() } },
      StringExpr { value, .. } => ExprNode::String { node: StringNode { val: value.clone() } },

      New { type_name } => ExprNode::New { node: NewNode { class: type_name.get_name() } },
      IsVoid { expr } => ExprNode::IsVoid { node: IsVoidNode { expr: ExprNode::from(expr) } },
    }
  }
}
