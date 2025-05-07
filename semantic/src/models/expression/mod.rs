mod assign;

use crate::models::Node;
use parser::model::expressions::Expression;

trait ExpressionNode: Node {
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum  ExpressionNode {
//     Add{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
//     Sub{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
//     Mul{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
//     Div{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
//     Equal{ left: Box<ExpressionNode>, right: Box<ExpressionNode> },
//     Not{ expr: Box<ExpressionNode> },
//     Negate{ expr: Box<ExpressionNode> },
// 
//     Int {val: i32},
//     Str {val: String},
//     Bool {val: bool},
// }



impl From<Expression> for dyn ExpressionNode {
    fn from(value: &Expression) -> Box<Self> {
        match value {
            Expression::PartialAssign { .. } | 
            Expression::PartialBinary { .. } |
            Expression::PartialDispatch { .. } |
            Expression::PartialCastDispatch { .. } => panic!("Should not have partial expressions"),
            Expression::Dispatch { calling_expr, fn_name, cast_type, param_list} => {
                
            }
            Expression::Assign { name, expr } => {
                let assign_expr = From::from(expr);
                AssignExprNode
            }
            Expression::Conditional { predicate, then_expr, else_expr } => {}
            Expression::Loop { predicate, body } => {}
            Expression::Case { switch_expression, branches } => {}
            Expression::Block { expr_list } => {}
            Expression::Let { let_init, in_expr } => {}
            Expression::Plus { left, right } => {}
            Expression::Minus { left, right } => {}
            Expression::Multiply { left, right } => {}
            Expression::Divide { left, right } => {}
            Expression::LessThan { left, right } => {}
            Expression::Equal { left, right } => {}
            Expression::LessThanOrEqual { left, right } => {}
            Expression::Negate { expr } => {}
            Expression::Not { expr } => {}
            Expression::IdentExpr { name } => {}
            Expression::IntExpr { value, line_num, line_pos } => {}
            Expression::BoolExpr { value, line_num, line_pos } => {}
            Expression::StringExpr { value, line_num, line_pos } => {}
            Expression::SelfTypeExpr { line_num, line_pos} => {}
            Expression::SelfExpr => {}
            Expression::New { type_name } => { todo!("check if type_name is a class") }
            Expression::IsVoid { expr } => {todo!("decorate expr")}
        }
    }
}

impl From<Box<Expression>> for dyn ExpressionNode {
    fn from(value: Box<Expression>) -> Box<Self> {
        todo!()
    }
}