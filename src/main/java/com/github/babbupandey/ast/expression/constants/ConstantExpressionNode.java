package com.github.babbupandey.ast.expression.constants;

import com.github.babbupandey.ast.expression.ExpressionNode;

public abstract class ConstantExpressionNode extends ExpressionNode {

    public enum ConstantType {
        BOOLEAN,
        STRING,
        INTEGER
    }

    private ConstantType type;

    public ConstantExpressionNode(ConstantType type) {
        super(ExpressionNodeType.LITERAL);
        this.type = type;
    }
}
