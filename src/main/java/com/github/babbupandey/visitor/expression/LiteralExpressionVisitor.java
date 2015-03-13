package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.constants.BooleanConstant;
import com.github.babbupandey.ast.expression.constants.ConstantExpressionNode;
import com.github.babbupandey.ast.expression.constants.IntegerConstant;
import com.github.babbupandey.ast.expression.constants.StringConstant;
import com.github.babbupandey.visitor.CoolParser;

public class LiteralExpressionVisitor extends ExpressionVisitor<ConstantExpressionNode> {
    @Override
    public ConstantExpressionNode visitLiteralExpr(CoolParser.LiteralExprContext ctx) {
        ConstantExpressionNode literal;
        if (ctx.False() != null) {
            literal = new BooleanConstant(false);
        } else if (ctx.True() != null) {
            literal = new BooleanConstant(true);
        } else if (ctx.StringLiteral() != null) {
            literal = new StringConstant(ctx.StringLiteral().getText());
        } else {
            literal = new IntegerConstant(Integer.parseInt(ctx.Integer().getText()));
        }
        return literal;
    }
}
