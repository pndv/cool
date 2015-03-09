package com.github.babbupandey.ast.expression;

public class NotExpressionNode extends ExpressionNode {
    private ExpressionNode expression;

    public NotExpressionNode(ExpressionNode expression) {
        super(ExpressionNodeType.NOT);
        this.expression = expression;
    }
}
