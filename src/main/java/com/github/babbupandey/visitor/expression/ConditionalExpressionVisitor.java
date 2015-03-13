package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ConditionalExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class ConditionalExpressionVisitor extends ExpressionVisitor<ConditionalExpressionNode> {
    @Override
    public ConditionalExpressionNode visitConditionalExpr(CoolParser.ConditionalExprContext ctx) {
        ExpressionNode ifExpression = new ExpressionVisitor<>().visitExpression(ctx.predicateExpr().expr());
        ExpressionNode thenExpression = new ExpressionVisitor<>().visitExpression(ctx.thenExpr().expr());
        ExpressionNode elseExpression = new ExpressionVisitor<>().visitExpression(ctx.elseExpr().expr());

        return new ConditionalExpressionNode(ifExpression, thenExpression, elseExpression);
    }
}
