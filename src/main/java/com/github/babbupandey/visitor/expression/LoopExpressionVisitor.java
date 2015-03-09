package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.LoopExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class LoopExpressionVisitor extends ExpressionVisitor<LoopExpressionNode> {
    @Override
    public LoopExpressionNode visitLoopExpr(CoolParser.LoopExprContext ctx) {
        return super.visitLoopExpr(ctx);
    }
}
