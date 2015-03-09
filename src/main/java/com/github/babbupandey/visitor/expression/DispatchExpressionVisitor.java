package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.DispatchExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class DispatchExpressionVisitor extends ExpressionVisitor<DispatchExpressionNode> {
    @Override
    public DispatchExpressionNode visitDispatchExpr(CoolParser.DispatchExprContext ctx) {
        return super.visitDispatchExpr(ctx);
    }
}
