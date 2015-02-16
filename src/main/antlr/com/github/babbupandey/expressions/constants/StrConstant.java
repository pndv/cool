package com.github.babbupandey.expressions.constants;

import com.github.babbupandey.exceptions.StringTooLongException;
import com.github.babbupandey.expressions.Expression;

public class StrConstant extends AbstractConstant<String>  {
    private String value;

    public StrConstant(String value) {
        super(value, ExpressionType.StrConstant);
        if(value.length() > 1024) {
            throw new StringTooLongException();
        }
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
