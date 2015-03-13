package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.DispatchExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;
import com.github.babbupandey.visitor.IdentifierVisitor;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class DispatchExpressionVisitor extends ExpressionVisitor<DispatchExpressionNode> {
    @Override
    public DispatchExpressionNode visitDispatchExpr(CoolParser.DispatchExprContext ctx) {
        ExpressionNode callExpression = new ExpressionVisitor<>().visitExpression(ctx.expr(0));
        TypeNode type = new TypeExpressionVisitor().visitType(ctx.type());
        IdentifierNode identifier = new IdentifierVisitor().visitIdentifier(ctx.identifier());
        List<ExpressionNode> parameterExpressionList = new ArrayList<>();
        parameterExpressionList.addAll(
                                        ctx.paramExpr()
                                           .stream()
                                           .map(e -> new ExpressionVisitor<>()
                                                             .visitExpression(e.expr()))
                                           .collect(Collectors.toList())
                                      );
        return new DispatchExpressionNode(callExpression, type, identifier, parameterExpressionList);
    }
}
