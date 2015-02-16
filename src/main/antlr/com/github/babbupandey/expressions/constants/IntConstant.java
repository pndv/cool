package com.github.babbupandey.expressions.constants;

import com.github.babbupandey.expressions.Expression;

public class IntConstant extends AbstractConstant<Integer> {
    private Integer value;

    public IntConstant(Integer value) {
        super(value, ExpressionType.IntConstant);
    }

    @Override
    public Integer evaluate() {
        return value;
    }

    @Override
    public ExpressionType getType() {
        return ExpressionType.IntConstant;
    }
}
