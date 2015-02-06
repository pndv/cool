grammar cool;


program
    : class+
    ;

class
    : CLASS type (INHERITS TYPE)? LCURL feature* RCURL SEMI
    ;
feature
    : id LAPREN formal (COMMA formal)* RPAREN COLON type LCURL expr RCURL
    | id COLON type (LARROW expr)?
    ;
formal
    : id COLON type
    ;
expr
    : ID LARROW expr
    | expr(AT type)? DOT id LPAREN expr (COMMA expr)* RPAREN
    | ID LPAREN expr (COMMA expr)* RPAREN
    | IF expr THEN expr ELSE expr FI
    | WHILE expr LOOP expr POOL
    | LCURL (expr)+ RCURL
    | LET ID COLON TYPE ( LARROW expr ) (COMMA ID COLON TYPE ( LARROW expr ))* IN expr
    | CASE expr OF (ID COLON TYPE RDARROW expr)+ ESAC
    | NEW TYPE
    | NOT expr
    | ISVOID expr
    | TILDE expr
    | LCURL expr RCURL
    | ID
    | integer
    | string
    | BOOL
    | expr (STAR|FSLASH) expr
    | expr (PLUS|MINUS) expr
    | expr (LE|LT|EQ) expr
    ;

BOOL
    : TRUE
    | FALSE;


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

CLASS : C L A S S ;
INHERITS : I N H E R I T S;
TYPE : T Y P E ;
ID : I D ;
IF : I F ;
THEN : T H E N ;
ELSE : E L S E ;
FI : F I ;
WHILE : W H I L E;
LOOP : L O O P ;
POOL : P O O L ;
LET : L E T ;
IN : I N ;
CASE : C A S E ;
OF : O F ;
ESAC : E S A C ;
NEW : N E W ;
ISVOID : I S V O I D;
TRUE : T R U E ;
FALSE : F A L S E;
NOT : N O T ;


STRING : '"' [^\r\t\n '"'

COMMENT_MULTILINE: '(*' .*? '*)' -> channel(HIDDEN);

WHITE_SPACE: [ \r\n\t] -> channel(HIDDEN);

fragment DIGIT : [0-9];
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