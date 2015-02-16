package com.github.babbupandey.expressions.constants;

import com.github.babbupandey.expressions.Expression;

public class BoolConstant extends AbstractConstant<Boolean> {

    private Boolean value;

    public BoolConstant(Boolean value) {
        super(value, ExpressionType.Arithmetic.BoolConstant);
    }

    @Override
    public Boolean evaluate() {
        return value;
    }

    @Override
    public ExpressionType getType() {
        return ExpressionType.Arithmetic.BoolConstant;
    }
}
