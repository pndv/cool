package com.github.babbupandey.ast.feature;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;

public class FormalNode {
    private IdentifierNode identifier;
    private TypeNode type;

    public FormalNode(IdentifierNode identifier, TypeNode type) {
        this.identifier = identifier;
        this.type = type;
    }
}
