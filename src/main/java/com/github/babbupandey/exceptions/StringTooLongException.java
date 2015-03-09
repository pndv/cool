package com.github.babbupandey.exceptions;

public class StringTooLongException extends CoolException {
    public StringTooLongException() {
        super("The length of input string exceeds 1024 characters");
    }
}
