package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ConditionalExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class ConditionalExpressionVisitor extends ExpressionVisitor<ConditionalExpressionNode> {
    @Override
    public ConditionalExpressionNode visitConditionalExpr(CoolParser.ConditionalExprContext ctx) {
        return super.visitConditionalExpr(ctx);
    }
}
