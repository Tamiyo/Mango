% Working on this, is ambiguous among other things
<mango>     🡆   <stmts>
<stmts>     🡆   <stmt> ENDL
<stmt>      🡆   <assign> | <expr>
<assign>    🡆   IDENT EQU <expr>
<expr>      🡆   VAR | <expr> + <expr> | <expr> - <expr> | <expr> * <expr> | <expr> / <expr>