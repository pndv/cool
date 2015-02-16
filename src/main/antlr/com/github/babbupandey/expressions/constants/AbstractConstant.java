package com.github.babbupandey.expressions.constants;

import com.github.babbupandey.expressions.Expression;

public abstract class AbstractConstant<T> extends Expression {

    public AbstractConstant(T value, Expression.ExpressionType type) {
        super(value, type);
    }
}
