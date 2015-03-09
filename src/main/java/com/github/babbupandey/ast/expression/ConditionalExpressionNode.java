package com.github.babbupandey.ast.expression;

public class ConditionalExpressionNode extends ExpressionNode {
    private ExpressionNode ifExpressionNode;
    private ExpressionNode thenExpressionNode;
    private ExpressionNode elseExpressionNode;

    public ConditionalExpressionNode(ExpressionNode ifExpressionNode, ExpressionNode thenExpressionNode, ExpressionNode elseExpressionNode) {
        super(ExpressionNodeType.CONDITIONAL);
        this.ifExpressionNode = ifExpressionNode;
        this.thenExpressionNode = thenExpressionNode;
        this.elseExpressionNode = elseExpressionNode;
    }
}
