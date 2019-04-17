NTS_MANGO                   NTS_STMTS

NTS_STMTS                   NTS_STMT TS_NEWLINE

NTS_STMT                    NTS_ASSIGN
                            | NTS_EXPR
                            | NTS_CTRL
NTS_ASSIGN                  TS_IDENT TS_EQUALS NTS_EXPR

NTS_EXPR                    TS_VARIABLE NTS_EXPR*
NTS_EXPR*                   NTS_OPERATOR NTS_EXPR NTS_EXPR*

NTS_CTRL                    NTS_CTRL_IF NTS_CTRL_ELSEIF NTS_CTRL_ELSE
NTS_CTRL_IF                 TS_IF TS_SPACE NTS_COND TS_SPACE TS_LCB NTS_STMTS TS_RCB
NTS_CTRL_ELSEIF             TS_ELSEIF TS_SPACE NTS_COND TS_SPACE TS_LCB NTS_STMTS TS_RCB
                            | e
NTS_CTRL_ELSE               TS_ELSE TS_SPACE TS_LCB NTS_STMTS TS_RCB
                            | e

NTS_COND                    TS_VARIABLE NTS_COND_OPERATOR TS_VARIABLE
                            | NTS_COND_S_OPERATOR TS_VARIABLE

NTS_OPERATOR                TS_PLUS
                            | TS_MINUS
                            | TS_MUL
                            | TS_DIV
                            | TS_EXP

NTS_COND_OPERATOR           TS_LT
                            | TS_LTE
                            | TS_GT
                            | TS_GTE
                            | TS_EQIV
                            | TS_TEQUIV

NTS_COND_S_OPERATOR         NEG
                            | NOTNULL

