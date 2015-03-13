package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.LoopExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class LoopExpressionVisitor extends ExpressionVisitor<LoopExpressionNode> {
    @Override
    public LoopExpressionNode visitLoopExpr(CoolParser.LoopExprContext ctx) {
        ExpressionNode predicate = new ExpressionVisitor<>().visitExpression(ctx.predicateExpr().expr());
        ExpressionNode loopConstruct = new ExpressionVisitor<>().visitExpression(ctx.expr());
        return new LoopExpressionNode(predicate, loopConstruct);
    }
}
