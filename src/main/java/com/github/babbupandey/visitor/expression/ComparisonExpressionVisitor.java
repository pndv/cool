package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ComparisonExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class ComparisonExpressionVisitor extends ExpressionVisitor<ComparisonExpressionNode> {
    @Override
    public ComparisonExpressionNode visitComparisonExpr(CoolParser.ComparisonExprContext ctx) {
        ExpressionNode leftExpression = visitExpression(ctx.expr());
        ExpressionNode rightExpression = visitExpression(ctx.rightExpr().expr());
        ComparisonExpressionNode.ComparisonType operator;
        if (ctx.EQ() != null) {
            operator = ComparisonExpressionNode.ComparisonType.EQ;
        } else if (ctx.LEQ() != null) {
            operator = ComparisonExpressionNode.ComparisonType.LEQ;
        } else {
            operator = ComparisonExpressionNode.ComparisonType.LT;
        }
        return new ComparisonExpressionNode(leftExpression, operator, rightExpression);
    }
}
