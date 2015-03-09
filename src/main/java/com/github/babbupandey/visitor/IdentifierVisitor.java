package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.IdentifierNode;

public class IdentifierVisitor extends CoolBaseVisitor<IdentifierNode> {
    @Override
    public IdentifierNode visitIdentifier(CoolParser.IdentifierContext ctx) {
        //todo
        return new IdentifierNode();
    }
}
