package com.github.babbupandey.visitor.expression;

import com.github.babbupandey.ast.IdentifierNode;
import com.github.babbupandey.ast.expression.ExpressionNode;
import com.github.babbupandey.ast.expression.IdentifierExpressionNode;
import com.github.babbupandey.exceptions.ExpressionNotFoundException;
import com.github.babbupandey.visitor.CoolBaseVisitor;
import com.github.babbupandey.visitor.CoolParser;
import com.github.babbupandey.visitor.IdentifierVisitor;

public class ExpressionVisitor<T extends ExpressionNode> extends CoolBaseVisitor<T> {

    public ExpressionNode visitExpression(CoolParser.ExprContext ctx) {
        ExpressionNode expression;

        if (ctx == null) {
            expression = null;
        } else if (ctx instanceof CoolParser.DispatchExprContext) {
            CoolParser.DispatchExprContext castCtx = (CoolParser.DispatchExprContext) ctx;
            expression = new DispatchExpressionVisitor().visitDispatchExpr(castCtx);
        } else if (ctx instanceof CoolParser.NewExprContext) {
            CoolParser.NewExprContext castCtx = (CoolParser.NewExprContext) ctx;
            expression = new NewExpressionVisitor().visitNewExpr(castCtx);
        } else if (ctx instanceof CoolParser.LetExprContext) {
            CoolParser.LetExprContext castCtx = (CoolParser.LetExprContext) ctx;
            expression = new LetExpressionVisitor().visitLetExpr(castCtx);
        } else if (ctx instanceof CoolParser.IdentExprContext) {
            CoolParser.IdentExprContext castCtx = (CoolParser.IdentExprContext) ctx;
            IdentifierNode identifier = new IdentifierVisitor().visitIdentifier(castCtx.identifier());
            expression = new IdentifierExpressionNode(identifier);
        } else if (ctx instanceof CoolParser.LoopExprContext) {
            CoolParser.LoopExprContext castCtx = (CoolParser.LoopExprContext) ctx;
            expression = new LoopExpressionVisitor().visitLoopExpr(castCtx);
        } else if (ctx instanceof CoolParser.ParenthesisedExprContext) {
            CoolParser.ParenthesisedExprContext castCtx = (CoolParser.ParenthesisedExprContext) ctx;
            expression = visitExpression(castCtx.expr());
        } else if (ctx instanceof CoolParser.NotExprContext) {
            CoolParser.NotExprContext castCtx = (CoolParser.NotExprContext) ctx;
            expression = new NotExpressionVisitor().visitNotExpr(castCtx);
        } else if (ctx instanceof CoolParser.MathsExprContext) {
            CoolParser.MathsExprContext castCtx = (CoolParser.MathsExprContext) ctx;
            expression = new ArithmeticExpressionVisitor().visitMathsExpr(castCtx);
        } else if (ctx instanceof CoolParser.IsVoidExprContext) {
            CoolParser.IsVoidExprContext castCtx = (CoolParser.IsVoidExprContext) ctx;
            expression = new IsVoidExpressionVisitor().visitIsVoidExpr(castCtx);
        } else if (ctx instanceof CoolParser.LiteralExprContext) {
            CoolParser.LiteralExprContext castCtx = (CoolParser.LiteralExprContext) ctx;
            expression = new LiteralExpressionVisitor().visitLiteralExpr(castCtx);
        } else if (ctx instanceof CoolParser.BlockExprContext) {
            CoolParser.BlockExprContext castCtx = (CoolParser.BlockExprContext) ctx;
            expression = new BlockExpressionVisitor().visitBlockExpr(castCtx);
        } else if (ctx instanceof CoolParser.CaseExprContext) {
            CoolParser.CaseExprContext castCtx = (CoolParser.CaseExprContext) ctx;
            expression = new CaseExpressionVisitor().visitCaseExpr(castCtx);
        } else if (ctx instanceof CoolParser.AssignExprContext) {
            CoolParser.AssignExprContext castCtx = (CoolParser.AssignExprContext) ctx;
            expression = new AssignExpressionVisitor().visitAssignExpr(castCtx);
        } else if (ctx instanceof CoolParser.TildeExprContext) {
            CoolParser.TildeExprContext castCtx = (CoolParser.TildeExprContext) ctx;
            expression = new TildeExpressionVisitor().visitTildeExpr(castCtx);
        } else if (ctx instanceof CoolParser.ConditionalExprContext) {
            CoolParser.ConditionalExprContext castCtx = (CoolParser.ConditionalExprContext) ctx;
            expression = new ConditionalExpressionVisitor().visitConditionalExpr(castCtx);
        } else if (ctx instanceof CoolParser.ComparisonExprContext) {
            CoolParser.ComparisonExprContext castCtx = (CoolParser.ComparisonExprContext) ctx;
            expression = new ComparisonExpressionVisitor().visitComparisonExpr(castCtx);
        } else {
            throw new ExpressionNotFoundException();
        }

        return expression;
    }

}
