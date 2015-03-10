package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.expression.BlockExpressionNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.visitor.CoolParser;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class BlockExpressionVisitor extends ExpressionVisitor<BlockExpressionNode> {
    @Override
    public BlockExpressionNode visitBlockExpr(CoolParser.BlockExprContext ctx) {
        List<ExpressionNode> expressionList = new ArrayList<>();
        expressionList.addAll(ctx.
                expr().
                stream().
                map(e -> new ExpressionVisitor<>().
                        visitExpression(e)).
                collect(Collectors.toList()));

        return new BlockExpressionNode(expressionList);
    }
}
