package com.github.babbupandey.ast.expressions.constants;

public class IntConstant extends AbstractConstant<Integer> {
    private Integer value;

    public IntConstant(Integer value) {
        super(ExpressionType.IntConstant);
        this.value = value;
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
