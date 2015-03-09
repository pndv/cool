package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.ClassNode;
import com.github.babbupandey.ast.ProgramNode;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class ProgramVisitor extends CoolBaseVisitor<ProgramNode> {

    private List<ClassNode> program;

    public ProgramVisitor() {
        program = new ArrayList<>();
    }

    @Override
    public ProgramNode visitProgram(CoolParser.ProgramContext ctx) {
        program.addAll(ctx.cool_class()
                               .stream()
                               .map(classContext -> new ClassVisitor().visitCool_class(classContext))
                               .collect(Collectors.toList()));
        return new ProgramNode(program);
    }
}
