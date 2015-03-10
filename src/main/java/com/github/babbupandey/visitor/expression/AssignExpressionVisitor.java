package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.expression.AssignmentExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;
import com.github.babbupandey.visitor.IdentifierVisitor;

public class AssignExpressionVisitor extends ExpressionVisitor<AssignmentExpressionNode> {

    @Override
    public AssignmentExpressionNode visitAssignExpr(CoolParser.AssignExprContext ctx) {
        IdentifierNode identifier = new IdentifierVisitor().visitIdentifier(ctx.identifier());
        ExpressionNode expression = new ExpressionVisitor<>().visitExpression(ctx.expr());
        return new AssignmentExpressionNode(identifier, expression);
    }
}
