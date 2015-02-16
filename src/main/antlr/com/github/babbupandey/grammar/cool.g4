grammar cool;
@header{com.github.babbupandey.parser;}

program
    : class+
    ;

class
    : Class TypeIdentifier (Inherits TypeIdentifier)? LCURL feature* RCURL SEMI
    ;
feature
    : Identifier LAPREN formal (COMMA formal)* RPAREN COLON TypeIdentifier LCURL expr RCURL
    | Identifier COLON TypeIdentifier (LARROW expr)?
    ;
formal
    : Identifier COLON TypeIdentifier
    ;
expr
    : Identifier LARROW expr
    | expr(AT TypeIdentifier)? DOT Identifier LPAREN expr (COMMA expr)* RPAREN
    | Identifier LPAREN expr (COMMA expr)* RPAREN
    | If expr Then expr Else expr Fi
    | While expr Loop expr Pool
    | LCURL (expr)+ RCURL
    | Let Identifier COLON TYPE ( LARROW expr ) (COMMA Identifier COLON TYPE ( LARROW expr ))* In expr
    | Case expr Of (Identifier COLON TYPE RDARROW expr)+ Esac
    | New TypeIdentifier
    | Not expr
    | IsVoid expr
    | TILDE expr
    | LCURL expr RCURL
    | Identifier
    | Integer
    | StringLiteral
    | True
    | False
    | expr (STAR|FSLASH) expr
    | expr (PLUS|MINUS) expr
    | expr (LE|LT|EQ) expr
    ;

PLUS : '+' ;
MINUS : '-' ;
STAR : '*' ;
FSLASH : '/' ;
TILDE : '~';
LT : '<' ;
LE : '>' ;
EQ : '=' ;
LEQ : '<=';
LCURL : '{';
RCURL : '}';
LAPREN : '(';
RPAREN : ')';
COMMA : ',';
COLON : ':';
SEMI : ';';
LARROW : '<-';
AT : '@' ;
DOT : '.' ;
RDARROW : '=>';

Class : C L A S S ;
Inherits : I N H E R I T S;
TYPE : T Y P E ;
If : I F ;
Then : T H E N ;
Else : E L S E ;
Fi : F I ;
While : W H I L E;
Loop : L O O P ;
Pool : P O O L ;
Let : L E T ;
In : I N ;
Case : C A S E ;
Of : O F ;
Esac : E S A C ;
New : N E W ;
IsVoid : I S V O I D;
True : T R U E ;
False : F A L S E;
Not : N O T ;

Identifier
    : TypeIdentifier
    | ObjectIdentifier
    ;

TypeIdentifier
    : [A-Z][a-zA-Z0-9_]*
    ;

ObjectIdentifier
    : [a-z][a-zA-Z0-9_]*
    ;

StringLiteral
    : '"' StringCharacters? '"'
    ;

Integer
    : OneToNine ZeroToNine*
    | ZeroToNine
    ;

NestedComment
    : '(*' .*? '*)' -> channel(HIDDEN)
    ;

SingleComment
    : '--' .*? '--'  -> channel(HIDDEN)
    ;

WHITE_SPACE
    : [ \r\n\t] -> channel(HIDDEN)
    ;

fragment StringCharacters
    :   StringCharacter+
    ;

fragment StringCharacter
    :   ~["\\]
    |   EscapeSequence
    ;

fragment EscapeSequence
    :   '\\' [btnfr"'\\]
    |   OctalEscape
    |   UnicodeEscape
    ;

fragment OctalEscape
    :   '\\' OctalDigit
    |   '\\' OctalDigit OctalDigit
    |   '\\' ZeroToThree OctalDigit OctalDigit
    ;

fragment UnicodeEscape
    :   '\\' 'u' HexDigit HexDigit HexDigit HexDigit
    ;

fragment OctalDigit
    :   [0-7]
    ;

fragment HexDigit
    :   [0-9a-fA-F]
    ;

fragment ZeroToThree
    :   [0-3]
    ;

fragment ZeroToNine
    : [0-9]
    ;

fragment OneToNine
    : [0-9]
    ;

fragment A : [aA];
fragment B : [bB];
fragment C : [cC];
fragment D : [dD];
fragment E : [eE];
fragment F : [fF];
fragment G : [gG];
fragment H : [hH];
fragment I : [iI];
fragment J : [jJ];
fragment K : [kK];
fragment L : [lL];
fragment M : [mM];
fragment N : [nN];
fragment O : [oO];
fragment P : [pP];
fragment Q : [qQ];
fragment R : [rR];
fragment S : [sS];
fragment T : [tT];
fragment U : [uU];
fragment V : [vV];
fragment W : [wW];
fragment X : [xX];
fragment Y : [yY];
fragment Z : [zZ];