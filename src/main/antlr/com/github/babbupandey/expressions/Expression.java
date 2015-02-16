package com.github.babbupandey.expressions;


public abstract class Expression<T> {
    private T value;
    private ExpressionType type;

    public abstract T evaluate();
    public abstract ExpressionType getType();
    public Expression(T value, ExpressionType type) {
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
