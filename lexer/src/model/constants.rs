use crate::model::token::Token;

pub const DOT: char = '.';
pub const AT: char = '@';
pub const TILDE: char = '~';
pub const STAR: char = '*';
pub const FORWARD_SLASH: char = '/';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';
pub const EQUAL: char = '=';
pub const DOUBLE_QUOTE: char = '"';
pub const SEMI_COLON: char = ';';
pub const COLON: char = ':';
pub const COMMA: char = ',';
pub const OPEN_PAREN: char = '(';
pub const CLOSE_PAREN: char = ')';
pub const OPEN_CURL: char = '{';
pub const CLOSE_CURL: char = '}';
pub const NULL_CHAR: char = '\0';
pub const CARRIAGE_RETURN: char = '\r';
pub const LINE_FEED: char = '\n';

pub const KEYWORD_CLASS: &str = "class";
pub const KEYWORD_INHERITS: &str = "inherits";
pub const KEYWORD_COND_IF_START: &str = "if";
pub const KEYWORD_COND_THEN: &str = "then";
pub const KEYWORD_COND_ELSE: &str = "else";
pub const KEYWORD_COND_IF_END: &str = "fi";
pub const KEYWORD_IN: &str = "in";
pub const KEYWORD_LET: &str = "let";
pub const KEYWORD_IS_VOID: &str = "isvoid";
pub const KEYWORD_NOT: &str = "not";
pub const KEYWORD_LOOP: &str = "loop";
pub const KEYWORD_LOOP_END: &str = "pool";
pub const KEYWORD_WHILE: &str = "while";
pub const KEYWORD_CASE_START: &str = "case";
pub const KEYWORD_CASE_END: &str = "esac";
pub const KEYWORD_NEW: &str = "new";
pub const KEYWORD_OF: &str = "of";
pub const KEYWORD_FALSE: &str = "false";
pub const KEYWORD_TRUE: &str = "true";
pub const KEYWORD_SELF_TYPE: &str = "SELF_TYPE";

pub const IDENT_TYPE: Token = Token::Ident {
    value: String::new(),
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const CASE_BRANCH_TYPE: Token = Token::CaseBranch {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const DOT_TYPE: Token = Token::Dot {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const COMMA_TYPE: Token = Token::Comma {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const ASSIGN_TYPE: Token = Token::Assign {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const COLON_TYPE: Token = Token::Colon {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const SEMI_COLON_TYPE: Token = Token::SemiColon {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const OPEN_PAREN_TYPE: Token = Token::OpenParen {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const CLOSE_PAREN_TYPE: Token = Token::CloseParen {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const OPEN_CURL_TYPE: Token = Token::OpenCurl {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const CLOSE_CURL_TYPE: Token = Token::CloseCurl {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const CLASS_TYPE: Token = Token::Class {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const INHERITS_TYPE: Token = Token::Inherits {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const IF_TYPE: Token = Token::If {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const THEN_TYPE: Token = Token::Then {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const ELSE_TYPE: Token = Token::Else {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const END_IF_TYPE: Token = Token::EndIf {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const LOOP_TYPE: Token = Token::Loop {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const END_LOOP_TYPE: Token = Token::EndLoop {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const IN_TYPE: Token = Token::In {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const OF_TYPE: Token = Token::Of {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const END_CASE_TYPE: Token = Token::EndCase {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const WHILE_TYPE: Token = Token::While {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const LET_TYPE: Token = Token::Let {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const CASE_TYPE: Token = Token::Case {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const NEW_TYPE: Token = Token::New {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const NOT_TYPE: Token = Token::Not {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const TILDE_TYPE: Token = Token::Tilde {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
pub const AT_TYPE: Token = Token::At {
    line_num: u32::MAX,
    line_pos: u32::MAX,
};
