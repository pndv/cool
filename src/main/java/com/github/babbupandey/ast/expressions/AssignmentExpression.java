package com.github.babbupandey.ast.expressions;

public class AssignmentExpression extends Expression {

    private String identifier;
    private Expression value;

    public AssignmentExpression(String identifier, Expression value) {
        super(ExpressionType.Assignment);
        this.identifier = identifier;
        this.value = value;
    }

    @Override
    public Object evaluate() {
        return value.evaluate();
    }

}
