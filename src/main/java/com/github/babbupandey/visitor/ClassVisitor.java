package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.ClassNode;
import com.github.babbupandey.ast.feature.FeatureNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.visitor.expression.TypeExpressionVisitor;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class ClassVisitor extends CoolBaseVisitor<ClassNode> {

    @Override
    public ClassNode visitCool_class(CoolParser.Cool_classContext ctx) {
        TypeNode classType = new TypeExpressionVisitor().visitType(ctx.type(0));
        TypeNode parentType = ctx.Inherits() != null ? new TypeExpressionVisitor().visitType(ctx.type(1)) : null;
        List<FeatureNode> featureList = null;
        if (ctx.feature() != null && ctx.feature().isEmpty()) {
            featureList = new ArrayList<>();
            featureList.addAll(ctx.feature()
                                       .parallelStream()
                                       .map(f -> f.attribute() != null ?
                                                         new FeatureVisitor().visitAttribute(f.attribute()) :
                                                         new FeatureVisitor().visitMethod(f.method()))
                                       .collect(Collectors.toList()));
        }
        return new ClassNode(classType, parentType, featureList);
    }
}
