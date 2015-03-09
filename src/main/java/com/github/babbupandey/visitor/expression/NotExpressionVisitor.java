package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.NotExpressionNode;
import com.github.babbupandey.visitor.CoolParser;
import com.github.babbupandey.visitor.expression.ExpressionVisitor;

public class NotExpressionVisitor extends ExpressionVisitor<NotExpressionNode> {
    @Override
    public NotExpressionNode visitNotExpr(CoolParser.NotExprContext ctx) {
        ExpressionNode expression = visitExpression(ctx.expr());
        return new NotExpressionNode(expression);
    }
}
