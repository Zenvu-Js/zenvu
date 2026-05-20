//! Advanced AST Parser & React JSX Compatibility Engine
//!
//! Parses hybrid Vue-like syntax and React JSX into a unified Intermediate Representation (IR).

use anyhow::{Result, bail};
use std::collections::HashMap;
use crate::lexer::{Token, tokenize};

#[derive(Debug, Clone)]
pub struct DirectiveNode {
    pub kind: String,       // "if", "for", "model", "on", "bind"
    pub argument: Option<String>,
    pub modifiers: Vec<String>,
    pub expression: String,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Element {
        tag: String,
        props: HashMap<String, String>,
        directives: Vec<DirectiveNode>,
        children: Vec<AstNode>,
        is_react_component: bool,
    },
    Text(String),
    ReactiveExpression(String), // e.g., {{count}}
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let tokens = tokenize(source).unwrap_or_default();
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    /// Parses the raw template into an Abstract Syntax Tree
    pub fn parse(&mut self) -> Result<AstNode> {
        tracing::info!("Parsing Zenvu Component AST...");
        
        let mut children = Vec::new();
        
        while let Some(tok) = self.peek() {
            match tok {
                Token::TemplateOpen { .. } => {
                    self.advance();
                    // Parse children until TemplateClose
                    while let Some(inner_tok) = self.peek() {
                        if matches!(inner_tok, Token::TemplateClose { .. }) {
                            self.advance();
                            break;
                        }
                        if let Ok(node) = self.parse_node() {
                            children.push(node);
                        } else {
                            self.advance();
                        }
                    }
                }
                Token::Eof => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }

        // Return root node
        if children.len() == 1 {
            Ok(children.pop().unwrap())
        } else {
            Ok(AstNode::Element {
                tag: "div".to_string(),
                props: {
                    let mut m = HashMap::new();
                    m.insert("class".to_string(), "zenvu-container".to_string());
                    m
                },
                directives: vec![],
                children,
                is_react_component: false,
            })
        }
    }

    fn parse_node(&mut self) -> Result<AstNode> {
        let tok = self.peek().ok_or_else(|| anyhow::anyhow!("Unexpected EOF"))?;
        match tok {
            Token::TagOpen(tag_name) => {
                let tag = tag_name.clone();
                self.advance();

                let mut props = HashMap::new();
                let mut directives = Vec::new();

                // Parse attributes & directives until TagEnd or TagSelfClose
                while let Some(attr_tok) = self.peek() {
                    match attr_tok {
                        Token::AttrName(name) => {
                            let name = name.clone();
                            self.advance();
                            let mut val = String::new();
                            if let Some(Token::AttrValue(v)) = self.peek() {
                                val = v.clone();
                                self.advance();
                            }
                            props.insert(name, val);
                        }
                        Token::Directive(dir) => {
                            directives.push(DirectiveNode {
                                kind: dir.kind.clone(),
                                argument: dir.argument.clone(),
                                modifiers: dir.modifiers.clone(),
                                expression: dir.expression.clone().unwrap_or_default(),
                            });
                            self.advance();
                        }
                        Token::TagEnd | Token::TagSelfClose => {
                            break;
                        }
                        _ => {
                            self.advance();
                        }
                    }
                }

                let self_closing = matches!(self.peek(), Some(Token::TagSelfClose));
                self.advance(); // consume TagEnd or TagSelfClose

                let mut children = Vec::new();
                if !self_closing {
                    // Parse children until closing tag matches
                    while let Some(child_tok) = self.peek() {
                        if let Token::TagClose(close_tag) = child_tok {
                            if close_tag == &tag {
                                self.advance(); // consume closing tag
                                break;
                            }
                        }
                        if let Ok(child) = self.parse_node() {
                            children.push(child);
                        } else {
                            self.advance();
                        }
                    }
                }

                Ok(AstNode::Element {
                    tag,
                    props,
                    directives,
                    children,
                    is_react_component: false,
                })
            }
            Token::Text(text) => {
                let text = text.clone();
                self.advance();
                Ok(AstNode::Text(text))
            }
            Token::Interpolation(expr) => {
                let expr = expr.clone();
                self.advance();
                Ok(AstNode::ReactiveExpression(expr))
            }
            _ => {
                self.advance();
                bail!("Unexpected token")
            }
        }
    }
}
