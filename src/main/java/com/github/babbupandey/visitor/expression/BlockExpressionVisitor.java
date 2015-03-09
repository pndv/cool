package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.BlockExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class BlockExpressionVisitor extends ExpressionVisitor<BlockExpressionNode> {
    @Override
    public BlockExpressionNode visitBlockExpr(CoolParser.BlockExprContext ctx) {
        return super.visitBlockExpr(ctx);
    }
}
