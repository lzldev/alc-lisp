#[allow(unused_macros)]
macro_rules! ast {
    () => {};
    {fn [$($args:ident),*] {$($body:tt),*}} => {
        crate::ast::Node::FunctionLiteral{
            token: crate::lexer::Token {
                value: "fn".into(),
                token_type: crate::lexer::TokenType::Word,
                ..Default::default()
            },
            arguments: vec![$(ast! { word $args}),*],
            body:Box::new($(ast! $body,)*)
        }
    };
    {expr [$($node:tt),*]} => {
        crate::ast::Node::Expression([
            $(ast! $node,)*
        ].into())
    };
    {list [$($node:tt),*]} => {
        crate::ast::Node::List([
            $(ast! $node,)*
        ].into())
    };
    {string $value:expr} => {
        crate::ast::Node::StringLiteral(crate::lexer::Token {
            value: ($value).into(),
            token_type: crate::lexer::TokenType::StringLiteral,
            ..Default::default()
        })
    };
    {number $value:ident} => {
        crate::ast::Node::NumberLiteral(crate::lexer::Token {
            value: ($value).into(),
            token_type: crate::lexer::TokenType::NumberLiteral,
            ..Default::default()
        })
    };
    {number $value:expr} => {
        crate::ast::Node::NumberLiteral{ value:$value,token:crate::lexer::Token {
        value: stringify!($value).into(),
        token_type: crate::lexer::TokenType::NumberLiteral,
        ..Default::default()
        } }
    };
    {word $value:ident} => {
        crate::ast::Node::Word(crate::lexer::Token {
            value: (stringify!($value)).into(),
            token_type: crate::lexer::TokenType::Word,
            ..Default::default()
        })
    };
    {word $value:expr} => {
        crate::ast::Node::Word(crate::lexer::Token {
            value: ($value).into(),
            token_type: crate::lexer::TokenType::Word,
            ..Default::default()
        })
    };
    {bool $value:ident} => {
        crate::ast::Node::BooleanLiteral(crate::lexer::Token {
            value: stringify!($value).into(),
            token_type: crate::lexer::TokenType::Word,
            ..Default::default()
        })
    };
    {bool $value:expr} => {
        crate::ast::Node::BooleanLiteral(crate::lexer::Token {
            value: (stringify!($value)).into(),
            token_type: crate::lexer::TokenType::Word,
            ..Default::default()
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Node,
        lexer::{Token, TokenType},
    };

    #[test]
    fn build_fn() {
        let some_fn = ast! { fn [hello] {{expr [{word "+"}, {number 1}, {number 2}]}}};

        let expected_fn = Node::FunctionLiteral {
            token: Token {
                value: "fn".into(),
                token_type: TokenType::Word,
                ..Default::default()
            },
            arguments: vec![Node::Word(Token {
                value: "hello".into(),
                token_type: TokenType::Word,
                ..Default::default()
            })],
            body: Box::new(Node::Expression(
                [
                    Node::Word(Token {
                        value: "+".into(),
                        token_type: TokenType::Word,
                        ..Default::default()
                    }),
                    Node::NumberLiteral {
                        value: 1,
                        token: Token {
                            value: "1".into(),
                            token_type: TokenType::NumberLiteral,
                            ..Default::default()
                        },
                    },
                    Node::NumberLiteral {
                        value: 2,
                        token: Token {
                            value: "2".into(),
                            token_type: TokenType::NumberLiteral,
                            ..Default::default()
                        },
                    },
                ]
                .into(),
            )),
        };

        assert_eq!(
            some_fn, expected_fn,
            "node resulted from the macro is not the same as the expected result"
        );
    }

    #[test]
    fn build_expr() {
        let some_expr = ast! { expr [
           {expr [{word "+"} , {number 1}, {number 1}]},
           {expr [{word "=="} , {bool true}, {bool false}]}
        ] };

        let expected_expr = Node::Expression(
            [
                Node::Expression(
                    [
                        Node::Word(Token {
                            value: "+".into(),
                            token_type: TokenType::Word,
                            ..Default::default()
                        }),
                        Node::NumberLiteral {
                            value: 1,
                            token: Token {
                                value: "1".into(),
                                token_type: TokenType::NumberLiteral,
                                ..Default::default()
                            },
                        },
                        Node::NumberLiteral {
                            value: 1,
                            token: Token {
                                value: "1".into(),
                                token_type: TokenType::NumberLiteral,
                                ..Default::default()
                            },
                        },
                    ]
                    .into(),
                ),
                Node::Expression(
                    [
                        Node::Word(Token {
                            value: "==".into(),
                            token_type: TokenType::Word,
                            ..Default::default()
                        }),
                        Node::BooleanLiteral(Token {
                            value: "true".into(),
                            token_type: TokenType::Word,
                            ..Default::default()
                        }),
                        Node::BooleanLiteral(Token {
                            value: "false".into(),
                            token_type: TokenType::Word,
                            ..Default::default()
                        }),
                    ]
                    .into(),
                ),
            ]
            .into(),
        );

        assert_eq!(
            some_expr, expected_expr,
            "node resulted from the macro is not the same as the expected result"
        );
    }
}
