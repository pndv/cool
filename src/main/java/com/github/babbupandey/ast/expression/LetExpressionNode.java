package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.TypeNode;
import java.util.AbstractMap;
import java.util.Map;

public class LetExpressionNode extends ExpressionNode {
    Map<IdentifierExpressionNode, AbstractMap.SimpleEntry<TypeNode, ExpressionNode>> idInitialisation;
    ExpressionNode inExpression;
    public LetExpressionNode(Map<IdentifierExpressionNode, AbstractMap.SimpleEntry<TypeNode, ExpressionNode>> idInitialisation,
                             ExpressionNode inExpression) {
        super(ExpressionNodeType.LET);
        this.idInitialisation = idInitialisation;
        this.inExpression = inExpression;
    }
}
