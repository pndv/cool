package com.github.babbupandey.ast.expression.constants;

import com.github.babbupandey.exceptions.StringTooLongException;

public class StringConstant extends ConstantExpressionNode {
    private static final int MAX_LENGTH = 1024;
    private String value;

    public StringConstant(String value) {
        super(ConstantType.STRING);
        if(value.length() > MAX_LENGTH) {
            throw new StringTooLongException();
        }
        this.value = value;
    }
}
