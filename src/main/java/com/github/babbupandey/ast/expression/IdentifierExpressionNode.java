package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.IdentifierNode;
import jdk.nashorn.internal.ir.IdentNode;

public class IdentifierExpressionNode extends ExpressionNode {
    private IdentifierNode identifier;
    public IdentifierExpressionNode(IdentifierNode identifier) {
        super(ExpressionNodeType.IDENTIFIER);
        this.identifier = identifier;
    }
}
