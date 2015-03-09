package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.CaseExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class CaseExpressionVisitor extends ExpressionVisitor<CaseExpressionNode> {
    @Override
    public CaseExpressionNode visitCaseExpr(CoolParser.CaseExprContext ctx) {
        return super.visitCaseExpr(ctx);
    }
}
