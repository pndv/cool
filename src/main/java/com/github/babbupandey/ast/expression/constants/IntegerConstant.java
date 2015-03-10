package com.github.babbupandey.ast.expression.constants;

public class IntegerConstant extends ConstantExpressionNode {
    private int value;

    public IntegerConstant(int value) {
        super(ConstantType.INTEGER);
        this.value = value;
    }

    public int getValue() {
        return value;
    }
}
