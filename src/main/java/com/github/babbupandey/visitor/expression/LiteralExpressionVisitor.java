package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.constants.ConstantExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class LiteralExpressionVisitor extends ExpressionVisitor<ConstantExpressionNode> {
    @Override
    public ConstantExpressionNode visitLiteralExpr(CoolParser.LiteralExprContext ctx) {
        return super.visitLiteralExpr(ctx);
    }
}
