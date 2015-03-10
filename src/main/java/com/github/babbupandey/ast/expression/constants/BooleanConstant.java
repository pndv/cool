package com.github.babbupandey.ast.expression.constants;

public class BooleanConstant extends ConstantExpressionNode {

    private boolean value;

    public BooleanConstant(boolean value) {
        super(ConstantType.BOOLEAN);
        this.value = value;
    }

    public boolean getValue() {
        return value;
    }
}
