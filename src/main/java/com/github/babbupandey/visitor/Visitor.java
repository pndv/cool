package com.github.babbupandey.visitor;

import com.github.babbupandey.ast.ProgramNode;

import java.util.ArrayList;
import java.util.List;

public class Visitor<T extends ProgramNode> extends CoolBaseVisitor<T> {

    private List<T> nodes;

    public Visitor() {
        nodes = new ArrayList<>();
    }

    @Override
    public T visitCool_class(CoolParser.Cool_classContext ctx) {
        visitType(ctx.type(0));
        if(ctx.Inherits() != null) {
            nodes.add(visitType(ctx.type(1)));
        }
        if (ctx.feature() != null && !ctx.feature().isEmpty()) {
            for(CoolParser.FeatureContext featureContext : ctx.feature()) {

            }
        }
        return super.visitCool_class(ctx);
    }

    @Override
    public T visitFeature(CoolParser.FeatureContext ctx) {
        return super.visitFeature(ctx);
    }

    @Override
    public T visitFormal(CoolParser.FormalContext ctx) {
        return super.visitFormal(ctx);
    }

    @Override
    public T visitConditionalExpr(CoolParser.ConditionalExprContext ctx) {
        return super.visitConditionalExpr(ctx);
    }

    @Override
    public T visitIdentExpr(CoolParser.IdentExprContext ctx) {
        return super.visitIdentExpr(ctx);
    }

    @Override
    public T visitNewExpr(CoolParser.NewExprContext ctx) {
        return super.visitNewExpr(ctx);
    }

    @Override
    public T visitDispatchExpr(CoolParser.DispatchExprContext ctx) {
        return super.visitDispatchExpr(ctx);
    }

    @Override
    public T visitLiteralExpr(CoolParser.LiteralExprContext ctx) {
        return super.visitLiteralExpr(ctx);
    }

    @Override
    public T visitMathsExpr(CoolParser.MathsExprContext ctx) {
        return super.visitMathsExpr(ctx);
    }

    @Override
    public T visitAssignExpr(CoolParser.AssignExprContext ctx) {
        return super.visitAssignExpr(ctx);
    }

    @Override
    public T visitTildeExpr(CoolParser.TildeExprContext ctx) {
        return super.visitTildeExpr(ctx);
    }

    @Override
    public T visitType(CoolParser.TypeContext ctx) {
        return super.visitType(ctx);
    }

    @Override
    public T visitIdentifier(CoolParser.IdentifierContext ctx) {
        return super.visitIdentifier(ctx);
    }

    @Override
    public T visitNotExpr(CoolParser.NotExprContext ctx) {
        return super.visitNotExpr(ctx);
    }

    @Override
    public T visitParenthesisedExpr(CoolParser.ParenthesisedExprContext ctx) {
        return super.visitParenthesisedExpr(ctx);
    }

    @Override
    public T visitBlockExpr(CoolParser.BlockExprContext ctx) {
        return super.visitBlockExpr(ctx);
    }

    @Override
    public T visitLetExpr(CoolParser.LetExprContext ctx) {
        return super.visitLetExpr(ctx);
    }

    @Override
    public T visitIsVoidExpr(CoolParser.IsVoidExprContext ctx) {
        return super.visitIsVoidExpr(ctx);
    }

    @Override
    public T visitCaseExpr(CoolParser.CaseExprContext ctx) {
        return super.visitCaseExpr(ctx);
    }

    @Override
    public T visitLoopExpr(CoolParser.LoopExprContext ctx) {
        return super.visitLoopExpr(ctx);
    }
}
