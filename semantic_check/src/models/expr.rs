enum ExprNode {
  Int {}
}


struct NewNode {class: String}
struct IsVoidNode {expr: ExprNode}
struct PlusNode {left: ExprNode, right: ExprNode}
struct MinusNode {left: ExprNode, right: ExprNode}
struct MultiplyNode {left: ExprNode, right: ExprNode}
struct DivNode {left: ExprNode, right: ExprNode}

struct NegateNode {expr: ExprNode}

struct LtNode {left: ExprNode, right: ExprNode}
struct LtEqNode {left: ExprNode, right: ExprNode}
struct EqNode {left: ExprNode, right: ExprNode}
struct NotNode {expr: ExprNode}

struct IdNode {name: String }
struct IntNode {val: i32}
struct StringNode {val: String}
struct TrueNode {}
struct FalseNode {}