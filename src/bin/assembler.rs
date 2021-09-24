use logos::{Lexer, Logos};
use std::{fs::File, io::Read};

fn parse_char_literal(lexer: &mut Lexer<Token>) -> String {
    return format!("'{}", lexer.slice().chars().nth(1).unwrap());
}

fn parse_decimal_literal(lexer: &mut Lexer<Token>) -> String {
    let value = lexer.slice().trim_start_matches("#");
    return format!("#{:02x}", value.parse::<u8>().unwrap());
}

fn parse_hex_literal(lexer: &mut Lexer<Token>) -> String {
    let value = lexer.slice().replace("$", "#");
    return value;
}

fn parse_binary_literal(lexer: &mut Lexer<Token>) -> String {
    let value = lexer.slice().trim_start_matches("b");
    return format!("#{:2x}", u8::from_str_radix(&value.to_string(), 2).unwrap());
}

macro_rules! handle_symmetric_binary_op {
    ($lexer:ident, $program:ident, $op:literal) => {
        if let Some(Token::Target(a)) = $lexer.next_if(|token| matches!(token, Token::Target(_))) {
            if let Some(Token::Target(b)) =
                $lexer.next_if(|token| matches!(token, Token::Target(_)))
            {
                match (a.as_str(), b.as_str()) {
                    ("p1", "p2") => $program.push_str($op),
                    ("p1", "s1") => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("p1", "s2") => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("p2", "p1") => $program.push_str($op),
                    ("p2", "s1") => {
                        $program.push_str("xX}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("p2", "s2") => {
                        $program.push_str("xXx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s1", "p1") => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s1", "p2") => {
                        $program.push_str("xX}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s1", "s2") => {
                        $program.push_str("X");
                        $program.push_str($op);
                        $program.push_str("}x{X}X{X");
                    }
                    ("s2", "p1") => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s2", "p2") => {
                        $program.push_str("xXx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s2", "s1") => {
                        $program.push_str("X");
                        $program.push_str($op);
                        $program.push_str("}x{X}X{X");
                    }
                    (a, b) => {
                        if a == b {
                            return Err(format!(
                                "Bad operation: {:?} {} {:?} - left may not be the same as right (consider duplicating)",
                                a, $op, a
                            ));
                        } else {
                            return Err(format!(
                                "Found targets {:?} and {:?} but failed to match them",
                                a, b
                            ));
                        }
                    }
                }
            } else if let Some(Token::Literal(b)) =
                $lexer.next_if(|token| matches!(token, Token::Literal(_)))
            {
                match a.as_str() {
                    "p1" => {
                        $program.push_str(&b);
                        $program.push_str($op);
                    }
                    "p2" => {
                        $program.push_str("x");
                        $program.push_str(&b);
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str(&b);
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str(&b);
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", a));
                    }
                }
            } else {
                match a.as_str() {
                    "p1" => {
                        return Err(format!(
                            "Bad operation: p1 (explicit) {} p1 (implicit)",
                            $op
                        ));
                    }
                    "p2" => {
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", a));
                    }
                }
            }
        } else if let Some(Token::Literal(a)) =
            $lexer.next_if(|token| matches!(token, Token::Literal(_)))
        {
            if let Some(Token::Target(b)) =
                $lexer.next_if(|token| matches!(token, Token::Target(_)))
            {
                match b.as_str() {
                    "p1" => {
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "p2" => {
                        $program.push_str("x");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", b));
                    }
                }
            } else if let Some(Token::Literal(b)) =
                $lexer.next_if(|token| matches!(token, Token::Literal(_)))
            {
                $program.push_str(&b);
                $program.push_str(&a);
                $program.push_str($op);
            } else {
                $program.push_str(&a);
                $program.push_str($op);
            }
        } else {
            $program.push_str($op);
        }
    };
}

macro_rules! handle_asymmetric_binary_op {
    ($lexer:ident, $program:ident, $op:literal) => {
        if let Some(Token::Target(a)) = $lexer.next_if(|token| matches!(token, Token::Target(_))) {
            if let Some(Token::Target(b)) =
                $lexer.next_if(|token| matches!(token, Token::Target(_)))
            {
                match (a.as_str(), b.as_str()) {
                    ("p1", "p2") => $program.push_str($op),
                    ("p1", "s1") => {
                        $program.push_str("X}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    ("p1", "s2") => {
                        $program.push_str("Xx}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    ("p2", "p1") => {
                        $program.push_str("x");
                        $program.push_str($op);
                    }
                    ("p2", "s1") => {
                        $program.push_str("xX}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    ("p2", "s2") => {
                        $program.push_str("xXx}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    ("s1", "p1") => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s1", "p2") => {
                        $program.push_str("xX}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s1", "s2") => {
                        $program.push_str("X");
                        $program.push_str($op);
                        $program.push_str("}x{X}X{X");
                    }
                    ("s2", "p1") => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s2", "p2") => {
                        $program.push_str("xXx}x{X}X{X");
                        $program.push_str($op);
                    }
                    ("s2", "s1") => {
                        $program.push_str("Xx");
                        $program.push_str($op);
                        $program.push_str("}x{X}X{X");
                    }
                    (a, b) => {
                        if a == b {
                            return Err(format!(
                                "Bad operation: {:?} {} {:?} - left may not be the same as right (consider duplicating)",
                                a, $op, a
                            ));
                        } else {
                            return Err(format!(
                                "Found targets {:?} and {:?} but failed to match them",
                                a, b
                            ));
                        }
                    }
                }
            } else if let Some(Token::Literal(b)) =
                $lexer.next_if(|token| matches!(token, Token::Literal(_)))
            {
                match a.as_str() {
                    "p1" => {
                        $program.push_str(&b);
                        $program.push_str("x");
                        $program.push_str($op);
                    }
                    "p2" => {
                        $program.push_str("x");
                        $program.push_str(&b);
                        $program.push_str("x");
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str(&b);
                        $program.push_str("x");
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str(&b);
                        $program.push_str("x");
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", a));
                    }
                }
            } else {
                match a.as_str() {
                    "p1" => {
                        return Err(format!(
                            "Bad operation: p1 (implicit) {} p1 (explicit)",
                            $op
                        ));
                    }
                    "p2" => {
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{Xx");
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", a));
                    }
                }
            }
        } else if let Some(Token::Literal(a)) =
            $lexer.next_if(|token| matches!(token, Token::Literal(_)))
        {
            if let Some(Token::Target(b)) =
                $lexer.next_if(|token| matches!(token, Token::Target(_)))
            {
                match b.as_str() {
                    "p1" => {
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "p2" => {
                        $program.push_str("x");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "s1" => {
                        $program.push_str("X}x{X}X{X");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    "s2" => {
                        $program.push_str("Xx}x{X}X{X");
                        $program.push_str(&a);
                        $program.push_str($op);
                    }
                    _ => {
                        return Err(format!("Found target {:?} but failed to match it", b));
                    }
                }
            } else if let Some(Token::Literal(b)) =
                $lexer.next_if(|token| matches!(token, Token::Literal(_)))
            {
                $program.push_str(&b);
                $program.push_str(&a);
                $program.push_str($op);
            } else {
                $program.push_str(&a);
                $program.push_str("x");
                $program.push_str($op);
            }
        } else {
            $program.push_str($op);
        }
    };
}

macro_rules! handle_unary_op {
    ($lexer:ident, $program:ident, $op:literal) => {
        if let Some(Token::Target(a)) = $lexer.next_if(|token| matches!(token, Token::Target(_))) {
            match a.as_str() {
                "p1" => {
                    $program.push_str($op);
                }
                "p2" => {
                    $program.push_str("x");
                    $program.push_str($op);
                }
                "s1" => {
                    $program.push_str("X}x{X}X{X");
                    $program.push_str($op);
                }
                "s2" => {
                    $program.push_str("Xx}x{X}X{X");
                    $program.push_str($op);
                }
                _ => {
                    return Err(format!("Found target {:?} but failed to match it", a));
                }
            }
        } else if let Some(Token::Literal(a)) =
            $lexer.next_if(|token| matches!(token, Token::Literal(_)))
        {
            $program.push_str(&a);
            $program.push_str($op);
        } else {
            $program.push_str($op);
        }
    };
}

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[token("add")]
    Add,
    #[token("sub")]
    Subtract,
    #[token("mul")]
    Multiply,
    #[token("div")]
    Divide,
    #[token("mod")]
    Modulo,
    #[token("neg")]
    #[token("inv")]
    BitwiseNegate,
    #[token("not")]
    LogicalNot,
    #[token("and")]
    BitwiseAnd,
    #[token("or")]
    BitwiseOr,
    #[token("xor")]
    BitwiseXor,
    #[token("eq")]
    Equal,
    #[token("lt")]
    Less,
    #[token("gt")]
    Greater,
    #[token("while")]
    #[token("loop")]
    LoopStart,
    #[token("nonzero")]
    LoopConditionNotZero,
    #[token("zero")]
    LoopConditionZero,
    #[token("cont")]
    #[token("next")]
    LoopContinue,
    #[token("dup")]
    Duplicate,
    #[token("drop")]
    Drop,
    #[token("swap")]
    Swap,
    #[token("values")]
    SwapValues,
    #[token("stacks")]
    SwapStacks,
    #[token("jz")]
    Conditional,
    #[token("jmp")]
    Jump,
    #[regex("[1-9]", |lex| lex.slice().parse::<u8>().unwrap())]
    JumpDistance(u8),
    #[token("push")]
    Push,
    #[regex(r"#[12]\d\d|#[1-9]\d|#\d", parse_decimal_literal)]
    #[regex(r"\$[0-9a-fA-F][0-9a-fA-F]", parse_hex_literal)]
    #[regex(r"b[01][01][01][01][01][01][01][01]", parse_binary_literal)]
    #[regex(r"'.'", parse_char_literal)]
    Literal(String),
    #[regex(r#""[^"]*""#, |v| v.slice().to_string())]
    StringLiteral(String),
    #[token("out")]
    #[token("pri")]
    Print,
    #[token("in")]
    #[token("get")]
    Input,
    #[token("st")]
    MoveToCell,
    #[token("ld")]
    LoadFromCell,
    #[token("p1", |v| v.slice().to_string())]
    #[token("p2", |v| v.slice().to_string())]
    #[token("s1", |v| v.slice().to_string())]
    #[token("s2", |v| v.slice().to_string())]
    Target(String),
    #[regex("#.*")]
    Comment,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[token("embed")]
    EmbeddedCode,
    #[error]
    #[regex(r"\w+", priority = 0)]
    Error,
}

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let name = args.next().unwrap();
    match args.next() {
        None => {
            println!("Usage: {} <file>", name);
            return Ok(());
        }
        Some(arg) => {
            let mut f = File::open(arg).map_err(|e| e.to_string())?;
            let mut out = String::new();
            f.read_to_string(&mut out).map_err(|e| e.to_string())?;
            let mut _lexer = Token::lexer(&out);
            let mut lexer = _lexer.by_ref().peekable();
            let mut program = String::new();
            while let Some(token) = lexer.next() {
                match token {
                    Token::Add => handle_symmetric_binary_op!(lexer, program, "+"),
                    Token::Subtract => handle_asymmetric_binary_op!(lexer, program, "-"),
                    Token::Multiply => handle_symmetric_binary_op!(lexer, program, "*"),
                    Token::Divide => handle_asymmetric_binary_op!(lexer, program, "/"),
                    Token::Modulo => handle_asymmetric_binary_op!(lexer, program, "%"),
                    Token::BitwiseNegate => handle_unary_op!(lexer, program, "~"),
                    Token::LogicalNot => handle_unary_op!(lexer, program, "!"),
                    Token::BitwiseAnd => handle_symmetric_binary_op!(lexer, program, "&"),
                    Token::BitwiseOr => handle_symmetric_binary_op!(lexer, program, "|"),
                    Token::BitwiseXor => handle_symmetric_binary_op!(lexer, program, "^"),
                    Token::Equal => handle_symmetric_binary_op!(lexer, program, "="),
                    Token::Less => handle_asymmetric_binary_op!(lexer, program, "<"),
                    Token::Greater => handle_asymmetric_binary_op!(lexer, program, ">"),
                    Token::LoopStart => match lexer.next() {
                        Some(Token::LoopConditionNotZero) => {
                            program.push_str("[");
                        }
                        Some(Token::LoopConditionZero) => {
                            program.push_str("(");
                        }
                        Some(_) => {
                            return Err(format!(
                                "Expected loop condition but found {:?}",
                                _lexer.slice()
                            ));
                        }
                        None => {
                            return Err(format!("Expected loop condition but found EOF"));
                        }
                    },
                    Token::LoopConditionNotZero => {
                        return Err(format!(
                            "Found unexpected loop condition {:?} while searching for a mnemonic",
                            _lexer.slice()
                        ))
                    }
                    Token::LoopConditionZero => {
                        return Err(format!(
                            "Found unexpected loop condition {:?} while searching for a mnemonic",
                            _lexer.slice()
                        ))
                    }
                    Token::LoopContinue => match lexer.next() {
                        Some(Token::LoopConditionNotZero) => {
                            program.push_str("]");
                        }
                        Some(Token::LoopConditionZero) => {
                            program.push_str(")");
                        }
                        Some(_) => {
                            return Err(format!(
                                "Expected loop condition but found {:?}",
                                _lexer.slice()
                            ));
                        }
                        None => {
                            return Err(format!("Expected loop condition but found EOF"));
                        }
                    },
                    Token::Duplicate => handle_unary_op!(lexer, program, ":"),
                    Token::Drop => handle_unary_op!(lexer, program, "`"),
                    Token::Swap => match lexer.next() {
                        Some(Token::SwapValues) => program.push_str("x"),
                        Some(Token::SwapStacks) => program.push_str("X"),
                        Some(_) => {
                            return Err(format!(
                                "Expected swap type but found {:?}",
                                _lexer.slice()
                            ));
                        }
                        None => {
                            return Err(format!("Expected swap type but found EOF"));
                        }
                    },
                    Token::SwapValues => {
                        return Err(format!(
                            "Found unexpected swap type {:?} while searching for a mnemonic",
                            _lexer.slice()
                        ))
                    }
                    Token::SwapStacks => {
                        return Err(format!(
                            "Found unexpected swap type {:?} while searching for a mnemonic",
                            _lexer.slice()
                        ))
                    }
                    Token::Conditional => handle_unary_op!(lexer, program, "?"),
                    Token::Jump => match lexer.next() {
                        Some(Token::JumpDistance(distance)) => {
                            program.push_str(&distance.to_string());
                        }
                        Some(a) => {
                            return Err(format!("Expected jump distance but found {:?}", a));
                        }
                        None => {
                            return Err(format!("Expected jump distance but found EOF"));
                        }
                    },
                    Token::JumpDistance(dist) => {
                        return Err(format!(
                            "Found unexpected jump distance {} while searching for a mnemonic",
                            dist
                        ))
                    }
                    Token::Push => match lexer.next() {
                        Some(Token::Literal(val)) => program.push_str(&val),
                        Some(Token::StringLiteral(val)) => {
                            program.push_str(&val.chars().rev().collect::<String>())
                        }
                        Some(_) => {
                            return Err(format!("Expected literal but found {:?}", _lexer.slice()));
                        }
                        None => {
                            return Err(format!("Expected literal but found EOF"));
                        }
                    },
                    Token::Literal(val) => {
                        return Err(format!(
                            "Found unexpected literal {} while searching for a mnemonic",
                            val
                        ))
                    }
                    Token::StringLiteral(val) => {
                        return Err(format!(
                            "Found unexpected string literal {:?} while searching for a mnemonic",
                            val
                        ))
                    }
                    Token::Target(target) => {
                        return Err(format!(
                            "Found unexpected target {:?} while searching for a mnemonic",
                            target
                        ))
                    }
                    Token::Print => handle_unary_op!(lexer, program, ";"),
                    Token::Input => program.push_str("@"),
                    Token::MoveToCell => handle_unary_op!(lexer, program, "{"),
                    Token::LoadFromCell => program.push_str("}"),
                    Token::Comment => {}
                    Token::Whitespace => {}
                    Token::EmbeddedCode => match lexer.next() {
                        Some(Token::StringLiteral(code)) => {
                            program.push_str(code.trim_matches('"'));
                        }
                        Some(_) => {
                            return Err(format!(
                                "Expected embedded code but found {:?}",
                                _lexer.slice()
                            ));
                        }
                        None => {
                            return Err(format!("Expected embedded code but found EOF"));
                        }
                    },
                    Token::Error => {
                        return Err(format!("Found invalid token {:?}", _lexer.slice()))
                    }
                }
            }

            println!("{}", program);

            return Ok(());
        }
    }
}
