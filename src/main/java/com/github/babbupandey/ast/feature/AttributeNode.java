package com.github.babbupandey.ast.feature;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.ExpressionNode;

public class AttributeNode extends FeatureNode {
    private IdentifierNode identifier;
    private TypeNode type;
    private ExpressionNode expression;

    public AttributeNode(IdentifierNode identifier, TypeNode type, ExpressionNode expression) {
        super(FeatureNodeType.ATTRIBUTE);
        this.identifier = identifier;
        this.type = type;
        this.expression = expression;
    }
}
