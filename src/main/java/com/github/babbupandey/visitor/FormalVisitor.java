package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.feature.FormalNode;
import com.github.babbupandey.visitor.expression.TypeExpressionVisitor;

public class FormalVisitor extends CoolBaseVisitor<FormalNode> {
    @Override
    public FormalNode visitFormal(CoolParser.FormalContext ctx) {
        IdentifierNode identifier = new IdentifierVisitor().visitIdentifier(ctx.identifier());
        TypeNode type = new TypeExpressionVisitor().visitType(ctx.type());
        return new FormalNode(identifier, type);
    }
}
