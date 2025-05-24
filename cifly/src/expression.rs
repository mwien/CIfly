use std::{collections::HashMap, error::Error, fmt};

use crate::instance::Sets;

// used pratt parser for expression parsing, inspired by:
// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

pub(crate) enum Expression {
    Atom(RuletableAtom),
    Junction(Op, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Junction(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub(crate) enum RuletableAtom {
    True,
    False,
    Current,
    Next,
    Set(usize),
}

impl RuletableAtom {
    fn get_identifier(&self) -> Option<&str> {
        match self {
            RuletableAtom::True => Some("true"),
            RuletableAtom::False => Some("false"),
            RuletableAtom::Current => Some("current"),
            RuletableAtom::Next => Some("next"),
            RuletableAtom::Set(_) => None,
        }
    }

    fn from_string(atom: &str, sets: &HashMap<String, usize>) -> RuletableAtom {
        let identifier_variants = [
            RuletableAtom::True,
            RuletableAtom::False,
            RuletableAtom::Current,
            RuletableAtom::Next,
        ];
        for variant in identifier_variants {
            if atom
                == variant
                    .get_identifier()
                    .expect("rule table atom should have an identifier")
            {
                return variant;
            }
        }
        RuletableAtom::Set(*sets.get(atom).unwrap())
    }
}

impl fmt::Display for RuletableAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuletableAtom::Set(s) => write!(f, "{}", s),
            _ => write!(
                f,
                "{}",
                self.get_identifier()
                    .expect("remaining rule table atoms should have an identifier"),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Op {
    And,
    In,
    Not,
    NotIn,
    Or,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::And => write!(f, "and"),
            Op::In => write!(f, "in"),
            Op::Not => write!(f, "not"),
            Op::NotIn => write!(f, "not in"),
            Op::Or => write!(f, "or"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParseExpressionError(String);

impl fmt::Display for ParseExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Expression Error: {}", self.0)
    }
}

impl Error for ParseExpressionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Expression {
    pub(crate) fn from_string(
        input: &str,
        sets: &HashMap<String, usize>,
    ) -> Result<Expression, ParseExpressionError> {
        let mut lexer = Lexer::new(input);
        let expression = Self::expr_bp(&mut lexer, 0, sets)?;
        expression.check()?;
        Ok(expression)
    }

    fn expr_bp(
        lexer: &mut Lexer,
        min_bp: u8,
        sets: &HashMap<String, usize>,
    ) -> Result<Expression, ParseExpressionError> {
        let mut lhs = match lexer.next() {
            Token::Atom(s) => Expression::Atom(RuletableAtom::from_string(&s, sets)),
            Token::BraceOpen => {
                let lhs = Self::expr_bp(lexer, 0, sets)?;
                if lexer.next() != Token::BraceClose {
                    return Err(ParseExpressionError(format!(
                        "missing closing brace for {}",
                        lhs
                    )));
                }
                lhs
            }
            Token::Op(Op::Not) => {
                let ((), r_bp) = Self::prefix_binding_power(Op::Not)?;
                let rhs = Self::expr_bp(lexer, r_bp, sets)?;
                Expression::Junction(Op::Not, vec![rhs])
            }
            Token::Op(op) => {
                return Err(ParseExpressionError(format!(
                    "unexpected infix operator: {}",
                    op
                )));
            }
            t => {
                return Err(ParseExpressionError(format!("unexpected {}", t)));
            }
        };

        loop {
            let op = match lexer.peek() {
                Token::Eof => break,
                Token::BraceClose => break,
                Token::Op(Op::Not) => {
                    return Err(ParseExpressionError(
                        "unexpected prefix operator 'not'".to_string(),
                    ))
                }
                Token::Op(op) => op,
                Token::Atom(s) => {
                    return Err(ParseExpressionError(format!(
                        "expected infix operator, closing brace or end of file, found {}",
                        s
                    )))
                }
                Token::BraceOpen => return Err(ParseExpressionError(
                    "expected infix operator, closing brace or end of file, found opening brace"
                        .to_string(),
                )),
            };

            let (l_bp, r_bp) = Self::infix_binding_power(op)?;
            if l_bp < min_bp {
                break;
            }

            lexer.next();
            let rhs = Self::expr_bp(lexer, r_bp, sets)?;
            lhs = Expression::Junction(op, vec![lhs, rhs]);
        }

        Ok(lhs)
    }

    fn prefix_binding_power(op: Op) -> Result<((), u8), ParseExpressionError> {
        match op {
            Op::Not => Ok(((), 3)),
            _ => Err(ParseExpressionError(format!(
                "expected prefix operator, found {}",
                op
            ))),
        }
    }

    fn infix_binding_power(op: Op) -> Result<(u8, u8), ParseExpressionError> {
        match op {
            Op::And | Op::Or => Ok((1, 2)),
            Op::In | Op::NotIn => Ok((5, 6)),
            _ => Err(ParseExpressionError(format!(
                "expected infix operator, found {}",
                op
            ))),
        }
    }

    pub(crate) fn check(&self) -> Result<(), ParseExpressionError> {
        match self {
            Expression::Atom(a) => match a {
                RuletableAtom::True | RuletableAtom::False => Ok(()),
                RuletableAtom::Current | RuletableAtom::Next => Err(ParseExpressionError(format!(
                    "found variable '{a}' when looking for operator"
                ))),
                RuletableAtom::Set(s) => Err(ParseExpressionError(format!(
                    "found set '{s}' when looking for operator"
                ))),
            },
            Expression::Junction(op, es) => match op {
                Op::And | Op::Or => {
                    let mut res = ();
                    for e in es.iter() {
                        res = e.check()?;
                    }
                    Ok(res)
                }
                Op::In | Op::NotIn => {
                    if es.len() < 2 {
                        return Err(ParseExpressionError(format!(
                            "operator '{op}' is a binary operator, right hand side is missing"
                        )));
                    }
                    match es[0] {
                        Expression::Atom(ref a) => {
                            match a {
                                RuletableAtom::Current | RuletableAtom::Next => (),
                                _ => return Err(ParseExpressionError(format!(
                                    "membership operator '{op}' expects '{}' or '{}' at the left, found {a}", RuletableAtom::Current, RuletableAtom::Next
                                )))
                            };
                        }
                        Expression::Junction(_, _) => {
                            return Err(ParseExpressionError(format!("membership operator '{op}' expects '{}' or '{}' at the left, found nested expression", RuletableAtom::Current, RuletableAtom::Next)));
                        }
                    }
                    match es[1] {
                        Expression::Atom(ref a) => match a {
                            RuletableAtom::Set(_) => (),
                            _ => {
                                return Err(ParseExpressionError(format!(
                                    "membership operator '{op}' expects set at the right, found {a}"
                                )))
                            }
                        },
                        Expression::Junction(_, _) => {
                            return Err(ParseExpressionError(format!(
                                "membership operator '{op}' expects set at the right, found nested expression"
                            )));
                        }
                    }
                    Ok(())
                }
                Op::Not => {
                    if es.len() != 1 {
                        Err(ParseExpressionError(format!(
                            "operator '{op}' is unary operator, found left and right hand side"
                        )))
                    } else {
                        es[0].check()
                    }
                }
            },
        }
    }

    pub(crate) fn evaluate(&self, sets: &Sets, v1: usize, v2: usize) -> bool {
        match self {
            Expression::Atom(a) => match a {
                RuletableAtom::True => true,
                RuletableAtom::False => false,
                _ => panic!("unexpected error: found '{a}' when looking for operator"),
            },
            Expression::Junction(op, es) => match op {
                Op::And => {
                    let mut res = true;
                    for e in es.iter() {
                        res &= e.evaluate(sets, v1, v2);
                    }
                    res
                }
                Op::Or => {
                    let mut res = false;
                    for e in es.iter() {
                        res |= e.evaluate(sets, v1, v2);
                    }
                    res
                }
                Op::In => {
                    let vertex = es[0].extract_vertex(v1, v2);
                    let set_id = es[1].extract_set_id();
                    sets.contains(set_id, vertex)
                }
                Op::Not => !es[0].evaluate(sets, v1, v2),
                Op::NotIn => {
                    let vertex = es[0].extract_vertex(v1, v2);
                    let set_id = es[1].extract_set_id();
                    !sets.contains(set_id, vertex)
                }
            },
        }
    }

    fn extract_vertex(&self, v1: usize, v2: usize) -> usize {
        match self {
            Expression::Atom(a) => match a {
                RuletableAtom::Current => v1,
                RuletableAtom::Next => v2,
                _ => panic!(
                    "unexpected error: found '{a}' instead of variable '{}' or '{}'",
                    RuletableAtom::Current,
                    RuletableAtom::Next
                ),
            },
            Expression::Junction(_, _) => {
                panic!(
                    "unexpected error: expected variable '{}' or '{}', found operator",
                    RuletableAtom::Current,
                    RuletableAtom::Next
                )
            }
        }
    }

    fn extract_set_id(&self) -> usize {
        match self {
            Expression::Atom(a) => match a {
                RuletableAtom::Set(idx) => *idx,
                _ => panic!("unexpected error: expected set, found {}", a),
            },
            Expression::Junction(_, _) => panic!("unexpected error: expected set, found operator"),
        }
    }
}

#[test]
fn test_parser() {
    let input = "current in Z and next in W";
    let sets = vec![("Z".to_owned(), 0), ("W".to_owned(), 1)];
    let s = Expression::from_string(input, &HashMap::from_iter(sets.into_iter())).unwrap();
    assert_eq!(s.to_string(), "(and (in current 0) (in next 1))");
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Atom(String),
    BraceOpen,
    BraceClose,
    Op(Op),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Atom(s) => write!(f, "{s}"),
            Token::BraceOpen => write!(f, "opening brace"),
            Token::BraceClose => write!(f, "closing brace"),
            Token::Op(op) => write!(f, "{op}"),
            Token::Eof => write!(f, "end of file"),
        }
    }
}

#[derive(Debug, Clone)]
struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = Vec::new();
        let mut currently_parsing = false;
        let mut current_token = "".to_owned();
        for c in input.chars() {
            if currently_parsing {
                if c.is_ascii_whitespace() {
                    tokens.push(Self::to_token(&current_token));
                    currently_parsing = false;
                    current_token.clear();
                } else if c == '(' {
                    tokens.push(Self::to_token(&current_token));
                    currently_parsing = false;
                    current_token.clear();
                    tokens.push(Token::BraceOpen);
                } else if c == ')' {
                    tokens.push(Self::to_token(&current_token));
                    currently_parsing = false;
                    current_token.clear();
                    tokens.push(Token::BraceClose);
                } else {
                    current_token.push(c);
                }
            } else {
                if c.is_ascii_whitespace() {
                    continue;
                }
                if c == '(' {
                    tokens.push(Token::BraceOpen);
                } else if c == ')' {
                    tokens.push(Token::BraceClose);
                } else {
                    currently_parsing = true;
                    current_token.push(c);
                }
            }
        }
        if !current_token.is_empty() {
            tokens.push(Self::to_token(&current_token));
        }
        let mut processed_tokens: Vec<Token> = Vec::new();
        // merge not in to Op::NotIn
        for (i, t) in tokens.iter().enumerate() {
            if i == 0 {
                processed_tokens.push(t.clone());
                continue;
            } else if (*processed_tokens.last().unwrap()) == Token::Op(Op::Not)
                && (*t) == Token::Op(Op::In)
            {
                processed_tokens.pop();
                processed_tokens.push(Token::Op(Op::NotIn));
            } else {
                processed_tokens.push(t.clone());
            }
        }
        processed_tokens.reverse();

        Lexer {
            tokens: processed_tokens,
        }
    }

    fn to_token(token: &str) -> Token {
        match token {
            "and" => Token::Op(Op::And),
            "in" => Token::Op(Op::In),
            "not" => Token::Op(Op::Not),
            "or" => Token::Op(Op::Or),
            _ => Token::Atom(token.to_string()),
        }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

#[test]
fn test_lexer() {
    let s = "current in C and not (current not in X or next not in V) or next in ancestors_old";
    let mut lexer = Lexer::new(s);
    assert_eq!(lexer.next(), Token::Atom("current".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::In));
    assert_eq!(lexer.next(), Token::Atom("C".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::And));
    assert_eq!(lexer.next(), Token::Op(Op::Not));
    assert_eq!(lexer.next(), Token::BraceOpen);
    assert_eq!(lexer.next(), Token::Atom("current".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::NotIn));
    assert_eq!(lexer.next(), Token::Atom("X".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::Or));
    assert_eq!(lexer.next(), Token::Atom("next".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::NotIn));
    assert_eq!(lexer.next(), Token::Atom("V".to_owned()));
    assert_eq!(lexer.next(), Token::BraceClose);
    assert_eq!(lexer.next(), Token::Op(Op::Or));
    assert_eq!(lexer.next(), Token::Atom("next".to_owned()));
    assert_eq!(lexer.next(), Token::Op(Op::In));
    assert_eq!(lexer.next(), Token::Atom("ancestors_old".to_owned()));
    assert_eq!(lexer.next(), Token::Eof);
}
