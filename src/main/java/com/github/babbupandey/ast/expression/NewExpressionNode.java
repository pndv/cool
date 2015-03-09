package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.TypeNode;

public class NewExpressionNode extends ExpressionNode {
    TypeNode type;

    public NewExpressionNode(TypeNode type) {
        super(ExpressionNodeType.NEW);
        this.type = type;
    }
}
