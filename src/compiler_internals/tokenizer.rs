#[derive(Debug, PartialEq)]
pub enum Keywords {
    Let,
    Const,
    Print,
    Loop,
    While,
    For,
    Function,
    If,
    Main,
    New,
    Import,
    StandardLibrary,
}

use Keywords::*;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Types {

    /// Bool type (false | true)
    Bool,
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

    ThreadHandle,

    TaskHandle,
}

use Types::*;

#[derive(Debug, PartialEq)]
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

    /// The object separator, for getting values and calling methods '.'
    ObjectSeparator,

    ImportSeparator,
}

use Delimiter::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keywords),
    Types(Types),
    Operator(Operators),
    Delimiter(Delimiter),
    Identifier(String),
    LiteralString(String),
    LiteralInteger(i64),
    LiteralUInteger(u64),
    LiteralFloat(f64),
}

pub fn test() {
    // let code = r#"
    // fn main() -> Result<(), str> {
    //     let [int] x = [2, 3, 4, 2];
    //     let int y = 2;
    //     let [bool] z;
    //
    //     x.for_each((a) => {
    //         print(a);
    //
    //         let bool result = y == a;
    //
    //         if result {
    //             z.push(result);
    //         }
    //     });
    //
    //     print("Right combination: {}", z);
    //
    // }
    //
    // "#
    // .to_string();

    let code = r#"

    import std::{ thread, ThreadError, ThreadHandle };
    import std::process;

    fn main() {

        let int z = 10;

        let Result<ThreadHandle, ThreadError> myThread = new thread(add_ten(z));

        let int output = match myThread.handle() {
            Ok(integer) => integer,
            Err(error) => {
                print("An error has occured when spawning a thread: {error}");
                process.exit(1);
            }
        };

        print("${output}");

    }

    fn add_ten(int y) -> int {

        let int x = y + 10;

        x

    }

    "#.to_string();

    tokenize(code);
}

/// Tokenizer *WIP*
pub fn tokenize(code: String) -> Vec<Token> {
    println!("{}", code);

    let mut tokens: Vec<Token> = Vec::new();

    let mut buffer: String = String::new();

    let mut inside_string = false;

    let mut inside_type_params = false;

    let mut counter = 0;

    for c in code.chars() {
        if inside_string {
            if c == '"' && !buffer.ends_with('\\') {
                inside_string = !inside_string;
                tokens.push(Token::LiteralString(buffer.clone()));
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
                ',' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    tokens.push(Token::Delimiter(Comma));
                },
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
                    let mut is_function_arrow = false;

                    if !buffer.is_empty() && buffer == "-" {
                        is_type_arrow = !is_type_arrow;
                        tokens.push(Token::Delimiter(TypeArrow));
                        buffer.clear();
                    } else if !buffer.is_empty() && buffer == "=" {
                        is_function_arrow = !is_function_arrow;
                        tokens.push(Token::Delimiter(FunctionArrow));
                        buffer.clear();
                    } else if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    if !is_type_arrow && !is_function_arrow {
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
                '[' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(OpenArray));
                }
                ']' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(CloseArray));
                }
                '=' => {

                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    if check_next_char(&code, &counter) == '>' {
                        buffer.push('=');
                    } else if check_next_char(&code, &counter) == '=' {
                        tokens.push(Token::Operator(Equals));
                    } else if let Token::Operator(Equals) = tokens.last().unwrap() {

                    } else {
                        tokens.push(Token::Operator(Assign));
                    }
                }
                '+' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Operator(Add));
                }
                '-' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    if check_next_char(&code, &counter) == '>' {
                        buffer.push('-');
                    } else {
                        tokens.push(Token::Operator(Subtract));
                    }
                }
                '*' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Operator(Multiply));
                }
                '/' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Operator(Divide));
                }
                '.' => {
                    let mut first_check = false;
                    let mut is_nan = false;

                    match buffer.parse::<i64>() {
                        Ok(_) => {
                            first_check = !first_check;
                        }
                        _ => {}
                    }

                    if !check_next_char(&code, &counter).is_alphabetic() && first_check {
                        is_nan = !is_nan;
                    }

                    if !buffer.is_empty() && !is_nan {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    if !is_nan {
                        tokens.push(Token::Delimiter(ObjectSeparator));
                    } else {
                        buffer.push(c);
                    }
                }
                ':' => {

                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }

                    if check_next_char(&code, &counter) == ':' {
                        buffer.push(':');
                        buffer.push(':');
                    }
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
                ',' => {
                    if !buffer.is_empty() {
                        process_buffer(&mut tokens, &buffer);
                        buffer.clear();
                    }
                    tokens.push(Token::Delimiter(Comma));
                }
                _ => {
                    buffer.push(c);
                }
            }
        }

        counter += 1;
    }

    if !buffer.is_empty() {
        process_buffer(&mut tokens, &buffer);
    }

    let mut counter = 0;

    for token in &tokens {
        counter += 1;
        println!("Token {}: {:?}", counter, token);
    }

    // println!("{tokens:?}");

    todo!()
}

fn check_next_char(code: &str, counter: &usize) -> char {
    code.chars().nth(*counter + 1).unwrap()
}

fn process_buffer(tokens: &mut Vec<Token>, buffer: &str) {
    match buffer {
        // "()" => {
        //     tokens.push(Token::Types())
        // }
        "::" => {
            tokens.push(Token::Delimiter(ImportSeparator));
        }
        "import" => {
            tokens.push(Token::Keyword(Import));
        }
        "std" => {
            tokens.push(Token::Keyword(StandardLibrary));
        }
        "main" => {
            tokens.push(Token::Keyword(Main));
        }
        "==" => {
            tokens.push(Token::Operator(Equals));
        }
        "=>" => {
            tokens.push(Token::Delimiter(FunctionArrow));
        }
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
        "bool" => {
            tokens.push(Token::Types(Bool));
        }
        "print" => {
            tokens.push(Token::Keyword(Print));
        }
        "+" => {
            tokens.push(Token::Operator(Add));
        }
        "-" => {
            tokens.push(Token::Operator(Subtract));
        }
        "*" => {
            tokens.push(Token::Operator(Multiply));
        }
        "/" => {
            tokens.push(Token::Operator(Divide));
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
