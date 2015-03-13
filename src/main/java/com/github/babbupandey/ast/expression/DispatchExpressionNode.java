package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.TypeNode;

import java.util.List;

public class DispatchExpressionNode extends ExpressionNode {
    private ExpressionNode callExpression;
    private TypeNode type;
    private IdentifierNode identifier;
    private List<ExpressionNode> parameterExpressionList;

    public DispatchExpressionNode(ExpressionNode callExpression,
                                  TypeNode type,
                                  IdentifierNode identifier,
                                  List<ExpressionNode> parameterExpressionList) {
        super(ExpressionNodeType.DISPATCH);
        this.callExpression = callExpression;
        this.type = type;
        this.identifier = identifier;
        this.parameterExpressionList = parameterExpressionList;
    }
}
