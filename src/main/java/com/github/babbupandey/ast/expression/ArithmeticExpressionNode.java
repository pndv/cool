package com.github.babbupandey.ast.expression;

public class ArithmeticExpressionNode extends ExpressionNode {
    public enum ArithmeticOperator {
        PLUS,
        MINUS,
        MULTIPLY,
        DIVIDE;
    }

    private ExpressionNode leftExpression;
    private ExpressionNode rightExpression;
    private ArithmeticOperator operator;

    public ArithmeticExpressionNode(ExpressionNode leftExpression, ArithmeticOperator operator, ExpressionNode rightExpression) {
        super(ExpressionNodeType.ARITHMETIC);
        this.leftExpression = leftExpression;
        this.operator = operator;
        this.rightExpression = rightExpression;
    }
}
