[src/lib.rs:18:5] &statements = [
    Var {
        name: "a",
        initializer: Some(
            Literal {
                value: Str(
                    "global",
                ),
            },
        ),
    },
    Block {
        statements: [
            Function {
                name: Token {
                    ttype: Identifier,
                    lexeme: "showA",
                    line: 3,
                },
                params: [],
                body: [
                    PrintStmt {
                        expr: Variable {
                            name: "a",
                            scope_depth: Cell {
                                value: None,
                            },
                        },
                    },
                ],
            },
            ExpressionStmt {
                expr: Call {
                    callee: Variable {
                        name: "showA",
                        scope_depth: Cell {
                            value: None,
                        },
                    },
                    paren: RightParen,
                    arguments: [],
                },
            },
            Var {
                name: "a",
                initializer: Some(
                    Literal {
                        value: Str(
                            "block",
                        ),
                    },
                ),
            },
            ExpressionStmt {
                expr: Call {
                    callee: Variable {
                        name: "showA",
                        scope_depth: Cell {
                            value: None,
                        },
                    },
                    paren: RightParen,
                    arguments: [],
                },
            },
        ],
    },
]