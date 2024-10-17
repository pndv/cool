use crate::model::constants::{
    KEYWORD_CASE_END, KEYWORD_CASE_START, KEYWORD_CLASS, KEYWORD_COND_ELSE, KEYWORD_COND_IF_END,
    KEYWORD_COND_IF_START, KEYWORD_COND_THEN, KEYWORD_FALSE, KEYWORD_IN, KEYWORD_INHERITS,
    KEYWORD_IS_VOID, KEYWORD_LET, KEYWORD_LOOP, KEYWORD_LOOP_END, KEYWORD_NEW, KEYWORD_NOT,
    KEYWORD_OF, KEYWORD_SELF_TYPE, KEYWORD_TRUE, KEYWORD_WHILE,
};
use std::fmt::{Display, Formatter};
use std::mem::discriminant;

#[derive(Eq, Debug, Clone)]
pub enum Token {
    Empty,
    EOF, //end of file
    Error {
        value: String,
        line_num: u32,
        line_pos: u32,
    },
    Comment {
        value: String,
        line_num: u32,
        line_pos: u32,
    },

    Ident {
        value: String,
        line_num: u32,
        line_pos: u32,
    },

    Dot {
        line_num: u32,
        line_pos: u32,
    },
    Comma {
        line_num: u32,
        line_pos: u32,
    },

    Assign {
        line_num: u32,
        line_pos: u32,
    }, // `<-`
    CaseBranch {
        line_num: u32,
        line_pos: u32,
    }, // `=>`

    At {
        line_num: u32,
        line_pos: u32,
    },
    Tilde {
        line_num: u32,
        line_pos: u32,
    },

    Plus {
        line_num: u32,
        line_pos: u32,
    },
    Minus {
        line_num: u32,
        line_pos: u32,
    },
    Star {
        line_num: u32,
        line_pos: u32,
    },
    ForwardSlash {
        line_num: u32,
        line_pos: u32,
    },

    LessOrEqual {
        line_num: u32,
        line_pos: u32,
    },
    Less {
        line_num: u32,
        line_pos: u32,
    },
    Equal {
        line_num: u32,
        line_pos: u32,
    },

    Colon {
        line_num: u32,
        line_pos: u32,
    },
    SemiColon {
        line_num: u32,
        line_pos: u32,
    },

    OpenParen {
        line_num: u32,
        line_pos: u32,
    },
    CloseParen {
        line_num: u32,
        line_pos: u32,
    },
    OpenCurl {
        line_num: u32,
        line_pos: u32,
    },
    CloseCurl {
        line_num: u32,
        line_pos: u32,
    },

    Class {
        line_num: u32,
        line_pos: u32,
    },
    Inherits {
        line_num: u32,
        line_pos: u32,
    },

    If {
        line_num: u32,
        line_pos: u32,
    },
    Then {
        line_num: u32,
        line_pos: u32,
    },
    Else {
        line_num: u32,
        line_pos: u32,
    },
    EndIf {
        line_num: u32,
        line_pos: u32,
    },

    While {
        line_num: u32,
        line_pos: u32,
    },
    Loop {
        line_num: u32,
        line_pos: u32,
    },
    EndLoop {
        line_num: u32,
        line_pos: u32,
    },

    Let {
        line_num: u32,
        line_pos: u32,
    },
    In {
        line_num: u32,
        line_pos: u32,
    },

    Case {
        line_num: u32,
        line_pos: u32,
    },
    Of {
        line_num: u32,
        line_pos: u32,
    },
    EndCase {
        line_num: u32,
        line_pos: u32,
    },

    New {
        line_num: u32,
        line_pos: u32,
    },
    IsVoid {
        line_num: u32,
        line_pos: u32,
    },
    Not {
        line_num: u32,
        line_pos: u32,
    },

    Int {
        value: i32,
        line_num: u32,
        line_pos: u32,
    },
    String {
        value: String,
        line_num: u32,
        line_pos: u32,
    },
    True {
        line_num: u32,
        line_pos: u32,
    },
    False {
        line_num: u32,
        line_pos: u32,
    },

    SelfType {
        line_num: u32,
        line_pos: u32,
    },
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Empty => write!(f, "Empty"),
            Token::EOF => write!(f, "EOF"),

            Token::Error {
                value,
                line_num,
                line_pos,
            } => write!(f, "{line_num}:{line_pos} Error [ {value} ]"),
            Token::Comment {
                value,
                line_num,
                line_pos,
            } => write!(f, "{line_num}:{line_pos} Comment [ {value} ]"),
            Token::Ident {
                value,
                line_num,
                line_pos,
            } => write!(f, "{line_num}:{line_pos} Ident [ {value} ]"),
            Token::String {
                value,
                line_num,
                line_pos,
            } => write!(f, "{line_num}:{line_pos} String [ {value} ]"),

            Token::Int {
                value,
                line_num,
                line_pos,
            } => write!(f, "{line_num}:{line_pos} Int [ {value} ]"),

            Token::Dot { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Dot"),
            Token::Comma { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Comma"),
            Token::Assign { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Assign"),
            Token::CaseBranch { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Lambda"),
            Token::At { line_num, line_pos } => write!(f, "{line_num}:{line_pos} At"),
            Token::Tilde { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Tilde"),
            Token::Plus { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Plus"),
            Token::Minus { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Minus"),
            Token::Star { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Star"),
            Token::ForwardSlash { line_num, line_pos } => {
                write!(f, "{line_num}:{line_pos} ForwardSlash")
            }
            Token::LessOrEqual { line_num, line_pos } => {
                write!(f, "{line_num}:{line_pos} LessOrEqual")
            }
            Token::Less { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Less"),
            Token::Equal { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Equal"),
            Token::Colon { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Colon"),
            Token::SemiColon { line_num, line_pos } => write!(f, "{line_num}:{line_pos} SemiColon"),
            Token::OpenParen { line_num, line_pos } => write!(f, "{line_num}:{line_pos} OpenParen"),
            Token::CloseParen { line_num, line_pos } => {
                write!(f, "{line_num}:{line_pos} CloseParen")
            }
            Token::OpenCurl { line_num, line_pos } => write!(f, "{line_num}:{line_pos} OpenCurl"),
            Token::CloseCurl { line_num, line_pos } => write!(f, "{line_num}:{line_pos} CloseCurl"),
            Token::Class { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Class"),
            Token::Inherits { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Inherits"),
            Token::If { line_num, line_pos } => write!(f, "{line_num}:{line_pos} If"),
            Token::Then { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Then"),
            Token::Else { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Else"),
            Token::EndIf { line_num, line_pos } => write!(f, "{line_num}:{line_pos} EndIf"),
            Token::While { line_num, line_pos } => write!(f, "{line_num}:{line_pos} While"),
            Token::Loop { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Loop"),
            Token::EndLoop { line_num, line_pos } => write!(f, "{line_num}:{line_pos} EndLoop"),
            Token::Let { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Let"),
            Token::In { line_num, line_pos } => write!(f, "{line_num}:{line_pos} In"),
            Token::Case { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Case"),
            Token::Of { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Of"),
            Token::EndCase { line_num, line_pos } => write!(f, "{line_num}:{line_pos} EndCase"),
            Token::New { line_num, line_pos } => write!(f, "{line_num}:{line_pos} New"),
            Token::IsVoid { line_num, line_pos } => write!(f, "{line_num}:{line_pos} IsVoid"),
            Token::Not { line_num, line_pos } => write!(f, "{line_num}:{line_pos} Not"),
            Token::True { line_num, line_pos } => write!(f, "{line_num}:{line_pos} True"),
            Token::False { line_num, line_pos } => write!(f, "{line_num}:{line_pos} False"),
            Token::SelfType { line_num, line_pos } => write!(f, "{line_num}:{line_pos} SelfType"),
        }
    }
}

impl Token {
    #[must_use]
    pub fn get_pos(&self) -> (u32, u32) {
        match self {
            Token::Empty | Token::EOF => (0, 0),
            Token::Error {
                line_num, line_pos, ..
            }
            | Token::Comment {
                line_num, line_pos, ..
            }
            | Token::Ident {
                line_num, line_pos, ..
            }
            | Token::Dot {
                line_num, line_pos, ..
            }
            | Token::Comma {
                line_num, line_pos, ..
            }
            | Token::Assign {
                line_num, line_pos, ..
            }
            | Token::CaseBranch {
                line_num, line_pos, ..
            }
            | Token::At {
                line_num, line_pos, ..
            }
            | Token::Tilde {
                line_num, line_pos, ..
            }
            | Token::Plus {
                line_num, line_pos, ..
            }
            | Token::Minus {
                line_num, line_pos, ..
            }
            | Token::Star {
                line_num, line_pos, ..
            }
            | Token::ForwardSlash {
                line_num, line_pos, ..
            }
            | Token::LessOrEqual {
                line_num, line_pos, ..
            }
            | Token::Less {
                line_num, line_pos, ..
            }
            | Token::Equal {
                line_num, line_pos, ..
            }
            | Token::Colon {
                line_num, line_pos, ..
            }
            | Token::SemiColon {
                line_num, line_pos, ..
            }
            | Token::OpenParen {
                line_num, line_pos, ..
            }
            | Token::CloseParen {
                line_num, line_pos, ..
            }
            | Token::OpenCurl {
                line_num, line_pos, ..
            }
            | Token::CloseCurl {
                line_num, line_pos, ..
            }
            | Token::Class {
                line_num, line_pos, ..
            }
            | Token::Inherits {
                line_num, line_pos, ..
            }
            | Token::If {
                line_num, line_pos, ..
            }
            | Token::Then {
                line_num, line_pos, ..
            }
            | Token::Else {
                line_num, line_pos, ..
            }
            | Token::EndIf {
                line_num, line_pos, ..
            }
            | Token::While {
                line_num, line_pos, ..
            }
            | Token::Loop {
                line_num, line_pos, ..
            }
            | Token::EndLoop {
                line_num, line_pos, ..
            }
            | Token::Let {
                line_num, line_pos, ..
            }
            | Token::In {
                line_num, line_pos, ..
            }
            | Token::Case {
                line_num, line_pos, ..
            }
            | Token::Of {
                line_num, line_pos, ..
            }
            | Token::EndCase {
                line_num, line_pos, ..
            }
            | Token::New {
                line_num, line_pos, ..
            }
            | Token::IsVoid {
                line_num, line_pos, ..
            }
            | Token::Not {
                line_num, line_pos, ..
            }
            | Token::Int {
                line_num, line_pos, ..
            }
            | Token::String {
                line_num, line_pos, ..
            }
            | Token::True {
                line_num, line_pos, ..
            }
            | Token::False {
                line_num, line_pos, ..
            }
            | Token::SelfType {
                line_num, line_pos, ..
            } => (*line_num, *line_pos),
        }
    }
    pub(crate) fn get_keyword(&self) -> Option<Token> {
        match self {
            Token::Ident {
                ref value,
                ref line_num,
                ref line_pos,
            } => {
                let lower_case = value.to_lowercase();
                let v = lower_case.as_str();

                match v {
                    KEYWORD_CLASS => Some(Token::Class {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_INHERITS => Some(Token::Inherits {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_COND_IF_START => Some(Token::If {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_COND_THEN => Some(Token::Then {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_COND_ELSE => Some(Token::Else {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_COND_IF_END => Some(Token::EndIf {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_IN => Some(Token::In {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_LET => Some(Token::Let {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_IS_VOID => Some(Token::IsVoid {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_NOT => Some(Token::Not {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_LOOP => Some(Token::Loop {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_LOOP_END => Some(Token::EndLoop {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_WHILE => Some(Token::While {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_CASE_START => Some(Token::Case {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_OF => Some(Token::Of {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_CASE_END => Some(Token::EndCase {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    KEYWORD_NEW => Some(Token::New {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_SELF_TYPE => Some(Token::SelfType {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_FALSE if value.starts_with('f') => Some(Token::False {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),
                    KEYWORD_TRUE if value.starts_with('t') => Some(Token::True {
                        line_num: *line_num,
                        line_pos: *line_pos,
                    }),

                    &_ => None,
                }
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn get_key(&self) -> &str {
        match self {
            Token::Empty => "Empty",
            Token::EOF => "[ EOF ]",
            Token::Error { .. } => "[ Error ]",
            Token::Comment { .. } => "[ Comment ]",
            Token::Ident { .. } => "[ Ident ]",
            Token::Dot { .. } => "[ . ]",
            Token::Comma { .. } => "[ , ]",
            Token::Assign { .. } => "[ <- ]",
            Token::CaseBranch { .. } => "[ => ]",
            Token::At { .. } => "[ @ ]",
            Token::Tilde { .. } => "[ ~ ]",
            Token::Plus { .. } => "[ + ]",
            Token::Minus { .. } => "[ - ]",
            Token::Star { .. } => "[ * ]",
            Token::ForwardSlash { .. } => "[ / ]",
            Token::LessOrEqual { .. } => "[ <= ]",
            Token::Less { .. } => "[ < ]",
            Token::Equal { .. } => "[ = ]",
            Token::Colon { .. } => "[ : ]",
            Token::SemiColon { .. } => "[ ; ]",
            Token::OpenParen { .. } => "[ ( ]",
            Token::CloseParen { .. } => "[ ) ]",
            Token::OpenCurl { .. } => "[ { ]",
            Token::CloseCurl { .. } => "[ } ]",
            Token::Class { .. } => "[ Class ]",
            Token::Inherits { .. } => "[ Inherits ]",
            Token::If { .. } => "[ if ]",
            Token::Then { .. } => "[ then ]",
            Token::Else { .. } => "[ else ]",
            Token::EndIf { .. } => "[ fi ]",
            Token::While { .. } => "[ while ]",
            Token::Loop { .. } => "[ loop ]",
            Token::EndLoop { .. } => "[ pool ]",
            Token::Let { .. } => "[ let ]",
            Token::In { .. } => "[ in ]",
            Token::Case { .. } => "[ case ]",
            Token::Of { .. } => "[ of ]",
            Token::EndCase { .. } => "[ esac ]",
            Token::New { .. } => "[ new ]",
            Token::IsVoid { .. } => "[ IsVoid ]",
            Token::Not { .. } => "[ not ]",
            Token::Int { .. } => "[ Int ]",
            Token::String { .. } => "[ String ]",
            Token::True { .. } => "[ True ]",
            Token::False { .. } => "[ False ]",
            Token::SelfType { .. } => "[ SELF_TYPE ]",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::constants::IDENT_TYPE;
    use crate::model::token::Token;

    #[test]
    fn test_equality() {
        let token1 = Token::Ident {
            value: String::from("Test1"),
            line_pos: 15,
            line_num: 40,
        };
        let token2 = Token::Ident {
            value: String::from("Test2"),
            line_pos: 25,
            line_num: 30,
        };

        assert!(token1 == token2);
        assert!(token1 == IDENT_TYPE);
    }
}
