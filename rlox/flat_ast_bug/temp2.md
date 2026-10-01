[src/lib.rs:18:5] &ast = Ast {
    expressions: [
        Literal {
            value: Str(
                "global",
            ),
        },
        Variable {
            name: "a",
            env_location: Cell {
                value: None,
            },
        },
        Variable {
            name: "showA",
            env_location: Cell {
                value: None,
            },
        },
        Call {
            callee: ExprId(
                2,
            ),
            paren: RightParen,
            arguments: [],
        },
        Literal {
            value: Str(
                "block",
            ),
        },
        Variable {
            name: "showA",
            env_location: Cell {
                value: None,
            },
        },
        Call {
            callee: ExprId(
                5,
            ),
            paren: RightParen,
            arguments: [],
        },
    ],
    statements: [
        Var {
            name: "a",
            initializer: Some(
                ExprId(
                    0,
                ),
            ),
            env_location: Cell {
                value: None,
            },
        },
        PrintStmt {
            expr: ExprId(
                1,
            ),
        },
        Function {
            name: Token {
                ttype: Identifier,
                lexeme: "showA",
                line: 3,
            },
            params: [],
            body: [
                StmtId(
                    1,
                ),
            ],
            env_location: Cell {
                value: None,
            },
        },
        ExpressionStmt {
            expr: ExprId(
                3,
            ),
        },
        Var {
            name: "a",
            initializer: Some(
                ExprId(
                    4,
                ),
            ),
            env_location: Cell {
                value: None,
            },
        },
        ExpressionStmt {
            expr: ExprId(
                6,
            ),
        },
        Block {
            statements: [
                StmtId(
                    2,
                ),
                StmtId(
                    3,
                ),
                StmtId(
                    4,
                ),
                StmtId(
                    5,
                ),
            ],
        },
    ],
}
