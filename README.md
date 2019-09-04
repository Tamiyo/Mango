# Mango
[Early Development] Mango is an imperative programming language written in Rust. This is my first language that I have written, and I am building the compiler from the ground up.

## Breakdown

### Lexicographic Analysis
The lexer is a __Non-Deterministic Finite Automaton (NFA)__ 5 Tuple defined as (Q, Σ, Δ, q0, F).
The lexer helps identify lexemes defined by the regular expressions below:

| LEXEME     |  REGEX PATTERN              |
| ---------- | --------------------------- |
| INTEGER    | [0-9]+                      |
| FLOAT      | [0-9]+.[0-9]+               |
| CHARACTER  | [a-zA-Z]                    |
| IDENTIFIER | [a-zA-Z]+                   |
| STRING     | \"[^\"]+\"                  |
| SYMBOL     | [+-*/%^=<>!(){}-,:;@.'"&$?] |

### Grammar
The __grammar__ is defined in [this file](python/grammar) using Backus-Naur Form Notation.

### Parser
This parser uses __Shift-Reduce Parsing__, in particular __CLR Parsing__ (Through there are plans to change it to SLR Parsing for a smaller parse table).

### Compiler Decisions
- The compiler is inherently _Object Oriented_.
- A _scoped symbol table_ is used to keep track of the different scopes within the program.
- The compiler takes advantage of _shallow variable binding_, that is the innermost calling function will decide what binding a variable is.
- The compiler uses dynamic typing.

### Syntax Examples
#### Variable Assignment
```
myvariable = "Hello World!"
print myvariable

myvariable = 12.0
print myvariable

>> Hello World!
15
```

#### Looping Constructs
###### For Loop Iteration Based
```
for i: 2 {
    print i
}

for i: 2..4 {
    print i
}

>> 0
1
2
3
```
###### For Loop Ranged Base
```
myarray = [2, 3, 7]

for i: myarray {
    print i
}

>> 2
3
7
```

###### While Loop
Note how there are no infinite loops. All loops must have some stopping criterion and cannot loop indefinitely.
```
x = 3
while x > 0 {
    print x
    x = x - 1
}

>> 3
2
1
```

#### Function Declaration and Usage
```
@myaddfunction: x, y {
    return x + y
}

@myprintfunction: {
    print "You got me!"
}

z = myaddfunction(1,2)
print z
myprintfunction()

>> 3
You got me!
```

#### Class Declaration and Usage
Class variables are created as they are called. You do not have to pre-declare the variables. Once you declare the variable, it is avaliable within the entire encompassing scope (not the global scope).
```
@myclass {
    @myclass: x {
        my.x = x
    }
    
    @printer() {
        print my.x
    }
}

z = myclass(2)
print z

>> 2
```

### NOTE
I would like to investigate __Rust__ as a potenttional option to convert the language to, but for now I will continue with __C++__.
