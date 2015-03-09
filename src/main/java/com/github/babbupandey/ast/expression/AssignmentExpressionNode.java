package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.IdentifierNode;

public class AssignmentExpressionNode extends ExpressionNode {
    private IdentifierNode identifier;
    private ExpressionNode expression;

    public AssignmentExpressionNode(IdentifierNode identifier, ExpressionNode expression) {
        super(ExpressionNodeType.ASSIGNMENT);
        this.identifier = identifier;
        this.expression = expression;
    }
}
