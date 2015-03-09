package com.github.babbupandey.ast.expression;

public class BlockExpressionNode extends ExpressionNode {

    private ExpressionNode expression;

    public BlockExpressionNode(ExpressionNode expression) {
        super(ExpressionNodeType.BLOCK);
        this.expression = expression;
    }
}
