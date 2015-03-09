package com.github.babbupandey.ast.expression;

public class ComparisonExpressionNode extends ExpressionNode {
    private ExpressionNode leftExpression;
    private ExpressionNode rightExpression;

    public ComparisonExpressionNode(ExpressionNode leftExpression,ExpressionNode rightExpression) {
        super(ExpressionNodeType.COMPARISON);
        this.leftExpression = leftExpression;
        this.rightExpression = rightExpression;
    }
}
