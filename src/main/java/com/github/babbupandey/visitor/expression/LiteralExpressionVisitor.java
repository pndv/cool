package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.LiteralExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class LiteralExpressionVisitor extends ExpressionVisitor<LiteralExpressionNode> {
    @Override
    public LiteralExpressionNode visitLiteralExpr(CoolParser.LiteralExprContext ctx) {
        return super.visitLiteralExpr(ctx);
    }
}
