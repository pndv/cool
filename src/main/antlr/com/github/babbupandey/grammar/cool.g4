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
UNDERSCORE : '_';

Class : c l a s s ;
Inherits : i n h e r i t s;
TYPE : t y p e;
If : i f ;
Then : t h e n;
Else : e l s e;
Fi : f i;
While : w h i l e;
Loop : l o o p;
Pool : p o o l;
Let : l e t;
In : i n;
Case : c a s e ;
Of : o f;
Esac : e s a c;
New : n e w;
IsVoid : i s v o i d;
True : t r u e;
False : f a l s e;
Not : n o t;
Self : s e l f;
Self_Type : S E L F UNDERSCORE T Y P E;

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
    : [1-9]
    ;

fragment a : 'a';
fragment b : 'b';
fragment c : 'c';
fragment d : 'd';
fragment e : 'e';
fragment f : 'f';
fragment g : 'g';
fragment h : 'h';
fragment i : 'i';
fragment j : 'j';
fragment k : 'k';
fragment l : 'l';
fragment m : 'm';
fragment n : 'n';
fragment o : 'o';
fragment p : 'p';
fragment q : 'q';
fragment r : 'r';
fragment s : 's';
fragment t : 't';
fragment u : 'u';
fragment v : 'v';
fragment w : 'w';
fragment x : 'x';
fragment y : 'y';
fragment z : 'z';

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