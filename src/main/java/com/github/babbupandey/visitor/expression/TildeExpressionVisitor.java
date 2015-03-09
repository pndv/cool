package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.TildeExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class TildeExpressionVisitor extends ExpressionVisitor<TildeExpressionNode> {
    @Override
    public TildeExpressionNode visitTildeExpr(CoolParser.TildeExprContext ctx) {
        ExpressionNode expression = visitExpression(ctx.expr());
        return new TildeExpressionNode(expression);
    }
}
