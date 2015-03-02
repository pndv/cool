package com.github.babbupandey.exceptions;

public class StringTooLongException extends RuntimeException {
    public StringTooLongException() {
        super("The length of input string exceeds 1024 characters");
    }
}
