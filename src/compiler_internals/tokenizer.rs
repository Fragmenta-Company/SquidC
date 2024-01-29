#[derive(Debug)]
pub enum Keywords {
    Let,
    Const,
    Print,
    Loop,
    While,
    Function,
}

use Keywords::*;

#[derive(Debug)]
pub enum Operators {
    /// Adding operator '+'
    Add,
    /// Subtracting operator '-'
    Subtract,
    /// Multiplying operator '*'
    Multiply,
    /// Dividing operator '/'
    Divide,
    /// Assigning operator '='
    Assign,
    /// Equals operator '=='
    Equals,
    /// Less than operator '<'
    LessThan,
    /// Greater than operator '>'
    GreaterThan,
}

use Operators::*;

#[derive(Debug)]
pub enum Types {
    /// Integer type (i64)
    ///
    /// Example: -32 | 32 | 64 | -64
    Int,
    /// Unsigned Integer type (u64)
    ///
    /// Example: 32 | 21 | 14
    UInt,
    /// Float type (f64)
    ///
    /// Example: 13.532 | -25.21 | 22.0
    Float,
    /// String type (UTF8/ASCII String)
    ///
    /// Example: "Hello, World!" | "Olá mundo!"
    StringType,
    /// Array type ([])
    ///
    /// Predefined type array, example:
    ///
    /// let [uint] x = [ 10, 21, 41, 743, 12 ];
    ///
    /// Array of unsigned integers
    Array,
    /// Object type ({})
    ///
    /// Example:
    ///
    /// let int x = 20;
    ///
    /// let obj object = { StatusCode: 10; Message: "Hello, World!"; RandomArray: x }
    Object,

    Unit,
}

use Types::*;

#[derive(Debug)]
pub enum Delimiter {
    /// End Line ';'
    Semicolon,
    /// Open Sequence '('
    OpenSequence,
    /// Close Sequence ')'
    CloseSequence,
    /// Open Array '['
    OpenArray,
    /// Close Array ']'
    CloseArray,
    /// Open Object '{'
    OpenObject,
    /// Close Object '}'
    CloseObject,
    /// String marking '"'
    QuoteString,
    /// Separate different items
    Comma,
    /// Declarates function return type '->'
    TypeArrow,
    /// Makes a function using '=>'
    ///
    /// Example: x.foreach((y) => { print(y); });
    FunctionArrow,

    OpenTypeParams,

    CloseTypeParams,
}

use Delimiter::*;

#[derive(Debug)]
pub enum Token {
    Keyword(Keywords),
    Types(Types),
    Operator(Operators),
    Delimiter(Delimiter),
    Identifier(String),
    Literal(String),
    LiteralInteger(i64),
    LiteralUInteger(u64),
    LiteralFloat(f64),
}

pub fn test() {
    let code = r#"
    fn function() -> Result<(), str> {
        const int INTEGER = 12;
        let str string = " Aloha + \" Chicago \" ";
        let str x = 10;
        let int y = 20;
        let int z = 30.30;
        print(x + 10);
    }

    fn function2() {

        const str STRINGERSON = "GEE GEE GEE GEE GEE";
        print(STRINGERSON);

    }
    "#
    .to_string();

    tokenize(code);
}

/// Tokenizer *WIP*
pub fn tokenize(code: String) -> Vec<Token> {
    println!("{}", code);

    let mut tokens: Vec<Token> = Vec::new();

    let mut buffer: String = String::new();

    let mut inside_string = false;

    let mut inside_type_params = false;

    for c in code.chars() {
        if inside_string {
            if c == '"' && !buffer.ends_with('\\') {
                inside_string = !inside_string;
                tokens.push(Token::Literal(buffer.clone()));
                buffer.clear();
            } else if c == '"' && buffer.ends_with('\\') {
                buffer.pop();
                buffer.push('"');
            } else {
                buffer.push(c);
            }
        } else if inside_type_params {
            if c == '>' {
                inside_type_params = !inside_type_params;
            }

            match c {
                ' ' | '\n' | '\t' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                }
                '(' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(OpenSequence));
                }
                ')' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(CloseSequence));
                }
                '>' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    tokens.push(Token::Delimiter(CloseTypeParams))
                }
                ',' => tokens.push(Token::Delimiter(Comma)),
                _ => {
                    buffer.push(c);
                }
            }
        } else {
            match c {
                ' ' | '\n' | '\t' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                }
                '<' => {
                    inside_type_params = !inside_type_params;

                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(OpenTypeParams))
                }
                '>' => {
                    let mut is_type_arrow = false;

                    if !buffer.is_empty() && buffer == "-" {
                        is_type_arrow = !is_type_arrow;
                        tokens.push(Token::Delimiter(TypeArrow));
                        buffer.clear();
                    } else if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    if !is_type_arrow {
                        tokens.push(Token::Delimiter(CloseTypeParams))
                    }
                }
                '{' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(OpenObject));
                }
                '}' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(CloseObject));
                }
                '(' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(OpenSequence));
                }
                ')' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(CloseSequence));
                }
                '+' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Operator(Add));
                }
                '=' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Operator(Assign));
                }
                ';' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(Semicolon));
                }
                '"' => {
                    inside_string = !inside_string;
                }
                _ => {
                    buffer.push(c);
                }
            }
        }
    }

    if !buffer.is_empty() {
        process_buffer(&mut tokens, &buffer);
    }

    for token in &tokens {
        println!("Token: {:?}", token);
    }

    todo!()
}

fn process_buffer(tokens: &mut Vec<Token>, buffer: &str) {
    match buffer {
        // "()" => {
        //     tokens.push(Token::Types())
        // }
        "->" => {
            tokens.push(Token::Delimiter(TypeArrow));
        }
        "fn" => {
            tokens.push(Token::Keyword(Function));
        }
        "const" => {
            tokens.push(Token::Keyword(Const));
        }
        "let" => {
            tokens.push(Token::Keyword(Let));
        }
        "int" => {
            tokens.push(Token::Types(Int));
        }
        "str" => {
            tokens.push(Token::Types(StringType));
        }
        "print" => {
            tokens.push(Token::Keyword(Print));
        }
        "+" => {
            tokens.push(Token::Operator(Add));
        }
        "=" => {
            tokens.push(Token::Operator(Assign));
        }
        ";" => {
            tokens.push(Token::Delimiter(Semicolon));
        }
        _ => {
            // Check if the buffer is a numeric literal
            if let Ok(value) = buffer.parse::<i64>() {
                tokens.push(Token::LiteralInteger(value));
            } else if let Ok(value) = buffer.parse::<u64>() {
                tokens.push(Token::LiteralUInteger(value));
            } else if let Ok(value) = buffer.parse::<f64>() {
                tokens.push(Token::LiteralFloat(value));
            } else {
                // Assume it's an identifier
                tokens.push(Token::Identifier(buffer.to_string()));
            }
        }
    }
}
