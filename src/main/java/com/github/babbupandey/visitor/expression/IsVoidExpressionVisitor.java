package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.IsVoidExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class IsVoidExpressionVisitor extends ExpressionVisitor<IsVoidExpressionNode> {
    @Override
    public IsVoidExpressionNode visitIsVoidExpr(CoolParser.IsVoidExprContext ctx) {
        ExpressionNode expression = new ExpressionVisitor<>().visitExpression(ctx.expr());
        return new IsVoidExpressionNode(expression);
    }
}
