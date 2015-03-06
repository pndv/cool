package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.ProgramNode;

public class Visitor<T extends ProgramNode> extends com.github.babbupandey.visitor.CoolBaseVisitor<T> {
    @Override
    public T visitCool_class(com.github.babbupandey.visitor.CoolParser.Cool_classContext ctx) {
        return super.visitCool_class(ctx);
    }

    @Override
    public T visitFeature(com.github.babbupandey.visitor.CoolParser.FeatureContext ctx) {
        return super.visitFeature(ctx);
    }

    @Override
    public T visitFormal(com.github.babbupandey.visitor.CoolParser.FormalContext ctx) {
        return super.visitFormal(ctx);
    }

    @Override
    public T visitExtendedIdentifierExpr(com.github.babbupandey.visitor.CoolParser.ExtendedIdentifierExprContext ctx) {
        return super.visitExtendedIdentifierExpr(ctx);
    }

    @Override
    public T visitComparisonExpr(com.github.babbupandey.visitor.CoolParser.ComparisonExprContext ctx) {
        return super.visitComparisonExpr(ctx);
    }

    @Override
    public T visitArithmeticExpr(com.github.babbupandey.visitor.CoolParser.ArithmeticExprContext ctx) {
        return super.visitArithmeticExpr(ctx);
    }

    @Override
    public T visitAssignmentExpr(com.github.babbupandey.visitor.CoolParser.AssignmentExprContext ctx) {
        return super.visitAssignmentExpr(ctx);
    }

    @Override
    public T visitLetExpr(com.github.babbupandey.visitor.CoolParser.LetExprContext ctx) {
        return super.visitLetExpr(ctx);
    }

    @Override
    public T visitCaseExpr(com.github.babbupandey.visitor.CoolParser.CaseExprContext ctx) {
        return super.visitCaseExpr(ctx);
    }

    @Override
    public T visitNewExpr(com.github.babbupandey.visitor.CoolParser.NewExprContext ctx) {
        return super.visitNewExpr(ctx);
    }

    @Override
    public T visitWhileExpr(com.github.babbupandey.visitor.CoolParser.WhileExprContext ctx) {
        return super.visitWhileExpr(ctx);
    }

    @Override
    public T visitIsVoidExpr(com.github.babbupandey.visitor.CoolParser.IsVoidExprContext ctx) {
        return super.visitIsVoidExpr(ctx);
    }

    @Override
    public T visitIfExpr(com.github.babbupandey.visitor.CoolParser.IfExprContext ctx) {
        return super.visitIfExpr(ctx);
    }

    @Override
    public T visitNotExpr(com.github.babbupandey.visitor.CoolParser.NotExprContext ctx) {
        return super.visitNotExpr(ctx);
    }

    @Override
    public T visitTildeExpr(com.github.babbupandey.visitor.CoolParser.TildeExprContext ctx) {
        return super.visitTildeExpr(ctx);
    }

    @Override
    public T visitIdentifierExpr(com.github.babbupandey.visitor.CoolParser.IdentifierExprContext ctx) {
        return super.visitIdentifierExpr(ctx);
    }

    @Override
    public T visitGroupExpr(com.github.babbupandey.visitor.CoolParser.GroupExprContext ctx) {
        return super.visitGroupExpr(ctx);
    }

    @Override
    public T visitParenthesisedExpr(com.github.babbupandey.visitor.CoolParser.ParenthesisedExprContext ctx) {
        return super.visitParenthesisedExpr(ctx);
    }

    @Override
    public T visitIdentifier(com.github.babbupandey.visitor.CoolParser.IdentifierContext ctx) {
        return super.visitIdentifier(ctx);
    }

    @Override
    public T visitType(com.github.babbupandey.visitor.CoolParser.TypeContext ctx) {
        return super.visitType(ctx);
    }
}
