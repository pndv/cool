use std::fmt::Display;
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
  pub(crate) expr: Box<ExprNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct MethodCallNode {
  pub(crate) expr: Box<ExprNode>,
  pub(crate) expr_cast_type: Option<String>, // Can only cast to types in its parent hierarchy
  pub(crate) method_name: String,
  pub(crate) args: Vec<Box<ExprNode>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ConditionalNode {
  pub(crate) condition: Box<ExprNode>,
  pub(crate) true_expr: Box<ExprNode>,
  pub(crate) false_expr: Box<ExprNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct WhileNode {
  pub(crate) condition: Box<ExprNode>,
  pub(crate) body: Box<ExprNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct ExprBlockNode {
  pub(crate) expressions: Vec<Box<ExprNode>>,
}
#[derive(Debug, Clone)]
pub(crate) struct LetInitNode {
  pub(crate) id: String,
  pub(crate) id_type: String,
  pub(crate) expr: Option<Box<ExprNode>>,
}

#[derive(Debug, Clone)]
pub(crate) struct LetNode {
  pub(crate) let_init: Vec<LetInitNode>,
  pub(crate) in_expr: Box<ExprNode>,
}
#[derive(Debug, Clone)]
pub(crate) struct CaseBranchNode {
  pub(crate) id: String,
  pub(crate) id_type: String,
  pub(crate) expr: Box<ExprNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct CaseNode {
  pub(crate) expr: Box<ExprNode>,
  pub(crate) branches: Vec<CaseBranchNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct NewNode { pub(crate) class: String }
#[derive(Debug, Clone)]
pub(crate) struct IsVoidNode { pub(crate) expr: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct PlusNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct MinusNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct MultiplyNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct DivNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct NegateNode { pub(crate) expr: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct LtNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct LtEqNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct EqNode { pub(crate) left: Box<ExprNode>, pub(crate) right: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct NotNode { pub(crate) expr: Box<ExprNode> }
#[derive(Debug, Clone)]
pub(crate) struct IdNode { pub(crate) name: String }
#[derive(Debug, Clone)]
pub(crate) struct IntNode { pub(crate) val: i32 }
#[derive(Debug, Clone)]
pub(crate) struct StringNode { pub(crate) val: String }
#[derive(Debug, Clone)]
pub(crate) struct BoolNode { pub(crate) val: bool }
#[derive(Debug, Clone)]
pub(crate) struct SelfTypeNode {}
#[derive(Debug, Clone)]
pub(crate) struct SelfNode {}
#[derive(Debug, Clone)]
pub(crate) struct TrueNode {}
#[derive(Debug, Clone)]
pub(crate) struct FalseNode {}
