package com.github.babbupandey.ast;

import com.github.babbupandey.exceptions.NoProgramBodyException;

import java.util.List;

public class ProgramNode {
    private List<ClassNode> classes;

    public ProgramNode(List<ClassNode> classes) {
        if (classes == null || classes.isEmpty()) {
            throw new NoProgramBodyException();
        }
        this.classes = classes;
    }
}
