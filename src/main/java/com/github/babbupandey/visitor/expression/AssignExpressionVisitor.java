package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.AssignmentExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class AssignExpressionVisitor extends ExpressionVisitor<AssignmentExpressionNode> {
    @Override
    public AssignmentExpressionNode visitAssignExpr(CoolParser.AssignExprContext ctx) {
        return super.visitAssignExpr(ctx);
    }
}
