NTS_MANGO -> NTS_STMTS

NTS_STMTS -> NTS_STMT TS_NEWLINE

NTS_STMT -> NTS_ASSIGN
NTS_STMT -> NTS_EXPR

NTS_ASSIGN -> TS_IDENT TS_EQUALS NTS_EXPR

NTS_EXPR -> TS_VARIABLE NTS_OPERATOR TS_VARIABLE

NTS_OPERATOR -> TS_PLUS
NTS_OPERATOR -> TS_MINUS
NTS_OPERATOR -> TS_MUL
NTS_OPERATOR -> TS_DIV
NTS_OPERATOR -> TS_EXP