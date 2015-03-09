package com.github.babbupandey.ast.expression;

public class TildeExpressionNode extends ExpressionNode {
    private ExpressionNode expression;

    public TildeExpressionNode(ExpressionNode expression) {
        super(ExpressionNode.ExpressionNodeType.TILDE);
        this.expression = expression;
    }
}
