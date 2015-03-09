package com.github.babbupandey.exceptions;

public class ExpressionNotFoundException extends CoolException {
    public ExpressionNotFoundException() {
        super("The expression could not be parsed"); //todo
    }
}
