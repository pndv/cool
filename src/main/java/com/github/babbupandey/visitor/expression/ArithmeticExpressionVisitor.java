package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.ArithmeticExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

public class ArithmeticExpressionVisitor extends ExpressionVisitor<ArithmeticExpressionNode> {
    @Override
    public ArithmeticExpressionNode visitMathsExpr(CoolParser.MathsExprContext ctx) {
        ExpressionNode leftExpression = visitExpression(ctx.expr());
        ExpressionNode rightExpression = visitExpression(ctx.rightExpr().expr());
        ArithmeticExpressionNode.ArithmeticOperator operator;
        if (ctx.FSLASH() != null) {
            operator = ArithmeticExpressionNode.ArithmeticOperator.DIVIDE;
        } else if (ctx.MINUS() != null) {
            operator = ArithmeticExpressionNode.ArithmeticOperator.MINUS;
        } else if (ctx.PLUS() != null) {
            operator = ArithmeticExpressionNode.ArithmeticOperator.PLUS;
        } else {
            operator = ArithmeticExpressionNode.ArithmeticOperator.MULTIPLY;
        }
        return new ArithmeticExpressionNode(leftExpression, operator, rightExpression);
    }
}
