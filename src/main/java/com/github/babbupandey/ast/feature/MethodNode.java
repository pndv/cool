package com.github.babbupandey.ast.feature;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;
import com.github.babbupandey.ast.expression.ExpressionNode;

import java.text.Normalizer;
import java.util.AbstractMap;
import java.util.List;

public class MethodNode extends FeatureNode {
    IdentifierNode methodName;
    List<FormalNode> parameterList;
    TypeNode methodType;
    ExpressionNode methodDefinition;

    public MethodNode(IdentifierNode methodName,
                      List<FormalNode> parameterList,
                      TypeNode methodType,
                      ExpressionNode methodDefinition) {
        super(FeatureNodeType.METHOD);
        this.methodName = methodName;
        this.methodType = methodType;
        this.parameterList = parameterList;
        this.methodDefinition = methodDefinition;
    }
}
