package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.IdentifierExpressionNode;
import com.github.babbupandey.ast.expression.LetExpressionNode;
import com.github.babbupandey.visitor.CoolParser;
import com.github.babbupandey.visitor.IdentifierVisitor;

import java.util.AbstractMap;
import java.util.HashMap;
import java.util.Map;

public class LetExpressionVisitor extends ExpressionVisitor<LetExpressionNode> {
    @Override
    public LetExpressionNode visitLetExpr(CoolParser.LetExprContext ctx) {
        Map<IdentifierExpressionNode, AbstractMap.SimpleEntry<TypeNode, ExpressionNode>> idInitialisation = new HashMap<>();
        ExpressionNode inExpression = new ExpressionVisitor<>().visitExpression(ctx.inExpr().expr());

        for(CoolParser.LetInitialisationContext c : ctx.letInitialisation()) {
            IdentifierExpressionNode id =
                    new IdentifierExpressionNode(new IdentifierVisitor().visitIdentifier(c.formal().identifier()));
            TypeNode type = new TypeNode();
            ExpressionNode expression = new ExpressionVisitor<>().visitExpression(c.expr());
            idInitialisation.put(id, new AbstractMap.SimpleEntry<>(type, expression));
        }

        return new LetExpressionNode(idInitialisation, inExpression);
    }
}
