package com.github.babbupandey.ast.expressions.constants;

import com.github.babbupandey.exceptions.StringTooLongException;

public class StrConstant extends AbstractConstant<String>  {
    private String value;

    public StrConstant(String value) {
        super(ExpressionType.StrConstant);
        if(value.length() > 1024) {
            throw new StringTooLongException();
        }
        this.value = value;
    }

    @Override
    public String evaluate() {
        return value;
    }

    @Override
    public ExpressionType getType() {
        return ExpressionType.StrConstant;
    }
}
