package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.feature.AttributeNode;
import com.github.babbupandey.ast.feature.FeatureNode;
import com.github.babbupandey.ast.feature.FormalNode;
import com.github.babbupandey.ast.feature.MethodNode;
import com.github.babbupandey.visitor.expression.ExpressionVisitor;
import com.github.babbupandey.visitor.expression.TypeExpressionVisitor;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class FeatureVisitor extends CoolBaseVisitor<FeatureNode> {

    @Override
    public FeatureNode visitMethod(CoolParser.MethodContext ctx) {
        IdentifierNode methodName = new IdentifierVisitor().visitIdentifier(ctx.identifier());
        List<FormalNode> parameterList = getFormalParameters(ctx.formal());
        TypeNode methodType = new TypeExpressionVisitor().visitType(ctx.type());
        ExpressionNode methodDefinition = new ExpressionVisitor<>().visitExpression(ctx.expr());

        return new MethodNode(methodName, parameterList, methodType, methodDefinition);
    }

    @Override
    public FeatureNode visitAttribute(CoolParser.AttributeContext ctx) {
        IdentifierNode identifier = new IdentifierVisitor().visitIdentifier(ctx.identifier());
        TypeNode type = new TypeExpressionVisitor().visitType(ctx.type());
        ExpressionNode expression = new ExpressionVisitor<>().visitExpression(ctx.expr()); // todo: ctx.expr() == null ? null : new Exp;

        return new AttributeNode(identifier, type, expression);
    }

    private List<FormalNode> getFormalParameters(List<CoolParser.FormalContext> ctx) {
        if(ctx == null || ctx.isEmpty()) {
            return null;
        }
        List<FormalNode> parameterList = new ArrayList<>();
        parameterList.addAll(ctx.stream().map(c -> new FormalVisitor().visitFormal(c)).collect(Collectors.toList()));
        return parameterList;
    }
}
