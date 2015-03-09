package com.github.babbupandey.ast.expression;

public class ComparisonExpressionNode extends ExpressionNode {

    private ExpressionNode leftExpression;
    private ExpressionNode rightExpression;
    private ComparisonType operator;
    public ComparisonExpressionNode(ExpressionNode leftExpression, ComparisonType operator, ExpressionNode rightExpression) {
        super(ExpressionNodeType.COMPARISON);
        this.leftExpression = leftExpression;
        this.rightExpression = rightExpression;
        this.operator = operator;
    }


    public enum ComparisonType {
        LT,
        EQ,
        LEQ
    }
}
