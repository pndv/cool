package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ArithmeticExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class ArithmeticExpressionVisitor extends ExpressionVisitor<ArithmeticExpressionNode> {
    @Override
    public ArithmeticExpressionNode visitMathsExpr(CoolParser.MathsExprContext ctx) {
        return super.visitMathsExpr(ctx);
    }
}
