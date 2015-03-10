package com.github.babbupandey.ast.expression;

import java.util.List;

public class BlockExpressionNode extends ExpressionNode {

    private List<ExpressionNode> expression;

    public BlockExpressionNode(List<ExpressionNode> expression) {
        super(ExpressionNodeType.BLOCK);
        this.expression = expression;
    }
}
