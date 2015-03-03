grammar cool;
@header{com.github.babbupandey.parser;}

program
    : cool_class+
    ;

cool_class
    : ClassKeyword TypeIdentifier (Inherits TypeIdentifier)? LCURL feature* RCURL SEMI
    ;

feature
    : identifier LPAREN formal (COMMA formal)* RPAREN COLON TypeIdentifier LCURL expr RCURL
    | identifier COLON TypeIdentifier (LARROW expr)? SEMI
    ;

formal
    : identifier COLON TypeIdentifier
    ;
expr
    : identifier LARROW expr
    | expr(AT TypeIdentifier)? DOT identifier LPAREN expr (COMMA expr)* RPAREN
    | identifier LPAREN expr (COMMA expr)* RPAREN
    | If expr Then expr Else expr Fi
    | While expr Loop expr Pool
    | LCURL (expr)+ RCURL
    | Let identifier COLON TYPE ( LARROW expr ) (COMMA identifier COLON TYPE ( LARROW expr ))* In expr
    | Case expr Of (identifier COLON TYPE RDARROW expr)+ Esac
    | New TypeIdentifier
    | Not expr
    | IsVoid expr
    | TILDE expr
    | LCURL expr RCURL
    | identifier
    | Integer
    | StringLiteral
    | True
    | False
    | expr (STAR|FSLASH) expr
    | expr (PLUS|MINUS) expr
    | expr (LE|LT|EQ) expr
    ;

identifier
    : TypeIdentifier
    | ObjectIdentifier
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
LPAREN : '(';
RPAREN : ')';
COMMA : ',';
COLON : ':';
SEMI : ';';
LARROW : '<-';
AT : '@' ;
DOT : '.' ;
RDARROW : '=>';
UNDERSCORE : '_';

ClassKeyword : S_c S_l S_a S_s S_s ;
Inherits : S_i S_n S_h S_e S_r S_i S_t S_s;
TYPE : S_t S_y S_p S_e;
If : S_i S_f ;
Then : S_t S_h S_e S_n;
Else : S_e S_l S_s S_e;
Fi : S_f S_i;
While : S_w S_h S_i S_l S_e;
Loop : S_l S_o S_o S_p;
Pool : S_p S_o S_o S_l;
Let : S_l S_e S_t;
In : S_i S_n;
Case : S_c S_a S_s S_e ;
Of : S_o S_f;
Esac : S_e S_s S_a S_c;
New : S_n S_e S_w;
IsVoid : S_i S_s S_v S_o S_i S_d;
True : S_t S_r S_u S_e;
False : S_f S_a S_l S_s S_e;
Not : S_n S_o S_t;
Self : S_s S_e S_l S_f;
Self_Type : S E L F UNDERSCORE T Y P E;



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
    : [1-9]
    ;

fragment S_a : 'a';
fragment S_b : 'b';
fragment S_c : 'c';
fragment S_d : 'd';
fragment S_e : 'e';
fragment S_f : 'f';
fragment S_g : 'g';
fragment S_h : 'h';
fragment S_i : 'i';
fragment S_j : 'j';
fragment S_k : 'k';
fragment S_l : 'l';
fragment S_m : 'm';
fragment S_n : 'n';
fragment S_o : 'o';
fragment S_p : 'p';
fragment S_q : 'q';
fragment S_r : 'r';
fragment S_s : 's';
fragment S_t : 't';
fragment S_u : 'u';
fragment S_v : 'v';
fragment S_w : 'w';
fragment S_x : 'x';
fragment S_y : 'y';
fragment S_z : 'z';

fragment A : 'A';
fragment B : 'B';
fragment C : 'C';
fragment D : 'D';
fragment E : 'E';
fragment F : 'F';
fragment G : 'G';
fragment H : 'H';
fragment I : 'I';
fragment J : 'J';
fragment K : 'K';
fragment L : 'L';
fragment M : 'M';
fragment N : 'N';
fragment O : 'O';
fragment P : 'P';
fragment Q : 'Q';
fragment R : 'R';
fragment S : 'S';
fragment T : 'T';
fragment U : 'U';
fragment V : 'V';
fragment W : 'W';
fragment X : 'X';
fragment Y : 'Y';
fragment Z : 'Z';