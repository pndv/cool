package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.NewExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class NewExpressionVisitor extends ExpressionVisitor<NewExpressionNode> {
    @Override
    public NewExpressionNode visitNewExpr(CoolParser.NewExprContext ctx) {
        TypeNode type = new TypeExpressionVisitor().visitType(ctx.type());
        return new NewExpressionNode(type);
    }
}
