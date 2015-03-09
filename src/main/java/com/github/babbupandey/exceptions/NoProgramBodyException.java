package com.github.babbupandey.exceptions;

public class NoProgramBodyException extends CoolException {

    public NoProgramBodyException() {
        super("The program does not have any body. Define at least one class in the program.");
    }
}
