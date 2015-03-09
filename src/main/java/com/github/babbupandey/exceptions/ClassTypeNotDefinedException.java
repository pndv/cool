package com.github.babbupandey.exceptions;

import com.github.babbupandey.ast.TypeNode;

public class ClassTypeNotDefinedException extends CoolException {
    public ClassTypeNotDefinedException() {
        super("The class type cannot be null");
    }

    public ClassTypeNotDefinedException(TypeNode typeNode) {
        super("The given type could not be found"); //todo: get the correct type name and make exception more informative
    }
}
