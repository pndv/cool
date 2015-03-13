package com.github.babbupandey.ast.expression;

public class LoopExpressionNode extends ExpressionNode {
    private ExpressionNode predicateExpr;
    private ExpressionNode loopConstruct;

    public LoopExpressionNode(ExpressionNode predicateExpr, ExpressionNode loopConstruct) {
        super(ExpressionNodeType.LOOP);
        this.loopConstruct = loopConstruct;
        this.predicateExpr = predicateExpr;
    }
}
