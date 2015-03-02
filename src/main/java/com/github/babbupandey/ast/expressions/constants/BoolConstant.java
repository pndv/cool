package com.github.babbupandey.ast.expressions.constants;

public class BoolConstant extends AbstractConstant<Boolean> {

    private Boolean value;

    public BoolConstant(Boolean value) {
        super(ExpressionType.Arithmetic.BoolConstant);
        this.value = value;
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
