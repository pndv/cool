package com.github.babbupandey.ast.expressions.constants;

import com.github.babbupandey.ast.expressions.Expression;

public abstract class AbstractConstant<T> extends Expression {

    public AbstractConstant(ExpressionType type) {
        super(value, type);
    }
}
