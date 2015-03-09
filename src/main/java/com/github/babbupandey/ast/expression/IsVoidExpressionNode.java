package com.github.babbupandey.ast.expression;

public class IsVoidExpressionNode extends ExpressionNode {

    private ExpressionNode expression;

    public IsVoidExpressionNode(ExpressionNode expression) {
        super(ExpressionNodeType.VOID);
        this.expression = expression;
    }
}
