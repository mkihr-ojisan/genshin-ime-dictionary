use std::{borrow::Cow, iter::Peekable, str::CharIndices};

#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    Text(Cow<'a, str>),
    Template {
        name: Cow<'a, str>,
        arguments: Vec<TemplateArgument<'a>>,
    },
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateArgument<'a> {
    pub name: Option<Cow<'a, str>>,
    pub value: Vec<Node<'a>>,
}

pub fn parse_mediawiki(input: &str) -> Vec<Node<'_>> {
    parse(tokenize(input))
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Text(&'a str),
    StartTemplate,
    ArgumentKeyValueSeparator,
    ArgumentSeparator,
    EndTemplate,
}

fn tokenize(text: &str) -> impl Iterator<Item = Token<'_>> + '_ {
    struct Tokenizer<'a> {
        text: &'a str,
        char_indices: Peekable<CharIndices<'a>>,
        next_token: Option<Token<'a>>,
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(token) = self.next_token.take() {
                return Some(token);
            }

            let start_index = self.char_indices.peek()?.0;

            loop {
                match self.char_indices.next() {
                    Some((i, '{')) => {
                        if matches!(self.char_indices.peek(), Some((_, '{'))) {
                            self.char_indices.next();
                            if start_index < i {
                                self.next_token = Some(Token::StartTemplate);
                                return Some(Token::Text(&self.text[start_index..i]));
                            } else {
                                return Some(Token::StartTemplate);
                            }
                        }
                    }
                    Some((i, '=')) => {
                        if start_index < i {
                            self.next_token = Some(Token::ArgumentKeyValueSeparator);
                            return Some(Token::Text(&self.text[start_index..i]));
                        } else {
                            return Some(Token::ArgumentKeyValueSeparator);
                        }
                    }
                    Some((i, '|')) => {
                        if start_index < i {
                            self.next_token = Some(Token::ArgumentSeparator);
                            return Some(Token::Text(&self.text[start_index..i]));
                        } else {
                            return Some(Token::ArgumentSeparator);
                        }
                    }
                    Some((i, '}')) => {
                        if matches!(self.char_indices.peek(), Some((_, '}'))) {
                            self.char_indices.next();
                            if start_index < i {
                                self.next_token = Some(Token::EndTemplate);
                                return Some(Token::Text(&self.text[start_index..i]));
                            } else {
                                return Some(Token::EndTemplate);
                            }
                        }
                    }
                    Some(_) => {}
                    None => {
                        if start_index < self.text.len() {
                            return Some(Token::Text(&self.text[start_index..]));
                        } else {
                            return None;
                        }
                    }
                }
            }
        }
    }

    Tokenizer {
        text,
        char_indices: text.char_indices().peekable(),
        next_token: None,
    }
}

fn parse<'a, T: Iterator<Item = Token<'a>>>(input: T) -> Vec<Node<'a>> {
    let mut input = input.peekable();
    parse_block(&mut input)
}

fn parse_block<'a, T: Iterator<Item = Token<'a>>>(input: &mut Peekable<T>) -> Vec<Node<'a>> {
    let mut nodes = Vec::new();

    loop {
        match input.next() {
            Some(Token::Text(text)) => {
                nodes.push(Node::Text(text.into()));
            }
            Some(Token::StartTemplate) => {
                nodes.push(parse_template(input));
            }
            Some(_) => {
                // unexpected token
            }
            None => {
                break;
            }
        }
    }

    nodes
}

fn parse_template<'a, T: Iterator<Item = Token<'a>>>(input: &mut Peekable<T>) -> Node<'a> {
    let name = if let Some(Token::Text(name)) = input.next() {
        name.trim()
    } else {
        return Node::Error;
    };

    let mut arguments = Vec::new();

    loop {
        match input.next() {
            Some(Token::ArgumentKeyValueSeparator) => {}
            Some(Token::EndTemplate) => {
                break;
            }
            Some(_) => {
                // unexpected token
            }
            None => {
                // unexpected end of input
                break;
            }
        }
        arguments.push(parse_template_argument(input));
    }

    Node::Template {
        name: name.into(),
        arguments,
    }
}

fn parse_template_argument<'a, T: Iterator<Item = Token<'a>>>(
    input: &mut Peekable<T>,
) -> TemplateArgument<'a> {
    let mut name = if let Some(Token::Text(name)) = input.peek() {
        let name = *name;
        input.next();
        Some(name)
    } else {
        None
    };

    let mut value = Vec::new();

    if let Some(Token::ArgumentKeyValueSeparator) = input.peek() {
        input.next();
    } else {
        if let Some(name) = name {
            value.push(Node::Text(name.into()));
        }
        name = None;
    }

    loop {
        match input.peek() {
            Some(Token::Text(text)) => {
                value.push(Node::Text((*text).into()));
                input.next();
            }
            Some(Token::StartTemplate) => {
                input.next();
                value.push(parse_template(input));
            }
            Some(Token::ArgumentSeparator | Token::EndTemplate) => {
                break;
            }
            Some(_) => {
                // unexpected token
                input.next();
            }
            None => {
                input.next();
                break;
            }
        }
    }

    TemplateArgument {
        name: name.map(|n| n.trim().into()),
        value,
    }
}

#[test]
fn test_tokenize() {
    let input = "aaa{aaa{}aa}aa{{aaa|{{bbb|ccc|ddd=eee}}}}{{}}";
    let mut token_iter = tokenize(input);

    assert_eq!(token_iter.next(), Some(Token::Text("aaa{aaa{}aa}aa")));
    assert_eq!(token_iter.next(), Some(Token::StartTemplate));
    assert_eq!(token_iter.next(), Some(Token::Text("aaa")));
    assert_eq!(token_iter.next(), Some(Token::ArgumentSeparator));
    assert_eq!(token_iter.next(), Some(Token::StartTemplate));
    assert_eq!(token_iter.next(), Some(Token::Text("bbb")));
    assert_eq!(token_iter.next(), Some(Token::ArgumentSeparator));
    assert_eq!(token_iter.next(), Some(Token::Text("ccc")));
    assert_eq!(token_iter.next(), Some(Token::ArgumentSeparator));
    assert_eq!(token_iter.next(), Some(Token::Text("ddd")));
    assert_eq!(token_iter.next(), Some(Token::ArgumentKeyValueSeparator));
    assert_eq!(token_iter.next(), Some(Token::Text("eee")));
    assert_eq!(token_iter.next(), Some(Token::EndTemplate));
    assert_eq!(token_iter.next(), Some(Token::EndTemplate));
    assert_eq!(token_iter.next(), Some(Token::StartTemplate));
    assert_eq!(token_iter.next(), Some(Token::EndTemplate));
    assert_eq!(token_iter.next(), None);
}

#[test]
fn test_parse() {
    let input = "aaa{aaa{}aa}aa{{aaa|{{bbb|ccc|ddd=eee}}}}";
    let tokens = tokenize(input);
    let nodes = parse(tokens);

    assert_eq!(
        nodes,
        vec![
            Node::Text("aaa{aaa{}aa}aa".into()),
            Node::Template {
                name: "aaa".into(),
                arguments: vec![TemplateArgument {
                    name: None,
                    value: vec![Node::Template {
                        name: "bbb".into(),
                        arguments: vec![
                            TemplateArgument {
                                name: None,
                                value: vec![Node::Text("ccc".into())]
                            },
                            TemplateArgument {
                                name: Some("ddd".into()),
                                value: vec![Node::Text("eee".into())]
                            }
                        ]
                    }]
                }]
            }
        ]
    );
}
