grammar cool;


program
    : class+
    ;

class
    : CLASS type (INHERITS TYPE)? LCURL feature* RCURL
    ;
feature
    : id LAPREN formal (COMMA formal)* RPAREN COLON type LCURL expr RCURL
    | id COLON type (LARROW expr)?
    ;
formal
    : id COLON type
    ;
expr
    : id LARROW expr
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
    | expr PLUS expr
    | expr MINUS expr
    | expr STAR expr
    | expr FSLASH expr
    | expr LT expr
    | expr LE expr
    | expr EQ expr
    | LCURL expr RCURL
    | ID
    | integer
    | string
    | TRUE
    | FALSE
    ;


BINOP
    : PLUS
    | MINUS
    | STAR
    | FSLASH
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
LARROW : '<-';
AT : '@' ;
DOT : '.' ;





CLASS : C L A S  S ;
INHERITS : I N H E R I T S;
TYPE : T Y P  E ;
ID : I D ;
IF : I F ;
THEN : T H E  N ;
ELSE : E L S  E ;
FI : F  I ;
WHILE : W H I L  E;
LOOP : L O O  P ;
POOL : P O O  L ;
LET : L E T ;
IN : I  N ;
CASE : C A S E ;
OF : O  F ;
ESAC : E S A C ;
NEW : N E  W ;
ISVOID : I S V O I D;
TRUE : T R U E ;
FALSE : F A L S E;
NOT : N O  T ;

RDARROW : R D A R RO W;
