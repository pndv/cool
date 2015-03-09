package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.IsVoidExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class IsVoidExpressionVisitor extends ExpressionVisitor<IsVoidExpressionNode> {
    @Override
    public IsVoidExpressionNode visitIsVoidExpr(CoolParser.IsVoidExprContext ctx) {
        return super.visitIsVoidExpr(ctx);
    }
}
