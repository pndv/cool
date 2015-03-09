package com.github.babbupandey.ast.expression;

import com.github.babbupandey.ast.feature.FormalNode;

import java.util.AbstractMap;
import java.util.List;
import java.util.Map;

public class CaseExpressionNode extends ExpressionNode {
    private ExpressionNode baseExpression;
    private List<AbstractMap.Entry<FormalNode, ExpressionNode>> caseList;

    public CaseExpressionNode(ExpressionNode baseExpression,
                              List<AbstractMap.Entry<FormalNode, ExpressionNode>> caseList) {
        super(ExpressionNodeType.CASE);
        this.baseExpression = baseExpression;
        this.caseList = caseList;
    }
}
