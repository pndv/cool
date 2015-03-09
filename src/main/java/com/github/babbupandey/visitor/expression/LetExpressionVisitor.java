package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.LetExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class LetExpressionVisitor extends ExpressionVisitor<LetExpressionNode> {
    @Override
    public LetExpressionNode visitLetExpr(CoolParser.LetExprContext ctx) {
        return super.visitLetExpr(ctx);
    }
}
