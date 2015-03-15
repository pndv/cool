package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.visitor.CoolBaseVisitor;
import com.github.babbupandey.visitor.CoolParser;

public class TypeExpressionVisitor extends CoolBaseVisitor<TypeNode> {

    @Override
    public TypeNode visitType(CoolParser.TypeContext ctx) {
        return new TypeNode(ctx.getText());
    }
}
