package com.github.babbupandey.ast.expression;

public abstract class ExpressionNode {
    public final ExpressionNodeType nodeType;

    public ExpressionNode(ExpressionNodeType nodeType) {
        this.nodeType = nodeType;
    }

    public enum ExpressionNodeType {
        ARITHMETIC,
        ASSIGNMENT,
        BLOCK,
        CASE,
        COMPARISON,
        CONDITIONAL,
        CONSTANT,
        DISPATCH,
        IDENTIFIER,
        VOID,
        LET,
        LOOP,
        NEW,
        NOT,
        LITERAL,
        TILDE
    }
}
