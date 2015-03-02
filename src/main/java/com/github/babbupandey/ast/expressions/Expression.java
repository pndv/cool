package com.github.babbupandey.ast.expressions;


public abstract class Expression<T> {
    private T value;
    private ExpressionType type;

    public abstract T evaluate();

    public ExpressionType getType() {
        return type;
    }

    public Expression(ExpressionType type) {
        this.value = value;
        this.type = type;
    }

    public static enum ExpressionType {
        BoolConstant,
        IntConstant,
        StrConstant,
        Identifier,
        Assignment,
        Dispatch,
        Conditional,
        Loop,
        Block,
        Let,
        Case,
        New,
        IsVoid,
        Arithmetic,
        Comparison,
    }

}
