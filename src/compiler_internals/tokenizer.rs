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

use async_std::task::JoinHandle;
use async_std::task;
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

        // Starts an immutable variable of type int (i64)
        let int z = 10;

        // Creates a new thread and run a function directly from it
        // A thread is a class given by the standard library
        // That's why you need the given syntax "new thread()"
        let Result<ThreadHandle, ThreadError> myThread = new thread(add_ten(z));

        // You can declare threads as this too:
        // let myThread = new thread();
        // And call a function like this:
        // myThread.run(function());
        // myThread.run(function2());
        // You can run multiple functions and at the end be given a vector of handles : )

        // Handles the thread function and sees if it's errored out while starting the thread
        let int output = match myThread.handle() {
            Ok(integer) => integer,
            Err(error) => {
                print("An error has occured when spawning a thread: ${error}");
                process.exit(1);
            }
        };

        // Outputs result number
        print("Output: ${output}");

        // Print test character
        print(";");

    }

    // Returns the given number + 10
    fn add_ten(int y) -> int {

        // Declare a variable that is a calculation.
        let int x = y + 10;

        // you don't need to type return since the variable is already at the end of the stack
        x

    }

    "#.to_string();

    let code_without_comments = remove_comments(&code);

    // println!("{code_without_comments}");

    let chuncks = code_chunckenizer(code_without_comments);
    let mut futures = Vec::new();

    async fn task_tokenize(chunck:String) -> Vec<Token> {
        tokenize(chunck)
    }

    for chunck in chuncks {
        let future = task::spawn(task_tokenize(chunck));
        futures.push(future);
    }

    let mut tokens = Vec::new();
    let mut counter_chunks = 0;

    for future in futures {
        counter_chunks+=1;
        async fn idk(future: JoinHandle<Vec<Token>>) -> Vec<Token> {
            future.await
        }

        tokens.extend(task::block_on(idk(future)));

    }

    // println!("\n\n");
    println!("Final tokens:\n");

    let mut counter = 0;
    for token in tokens {
        println!("Token {counter}: {token:?}");
        counter+=1;
    }

    println!("Chuncks of code: {counter_chunks}");

}

fn remove_comments(source_code: &str) -> String {
    let mut code_without_comments = String::new();

    for line in source_code.lines() {
        // Check for single-line comments (//) and ignore the rest of the line
        if let Some(index) = line.find("//") {
            let line_without_comment = &line[0..index];
            code_without_comments.push_str(line_without_comment.trim());
        } else {
            let line_without_leading_whitespace = line.trim_start();
            code_without_comments.push_str(line_without_leading_whitespace);
        }

        // Add a newline character after each processed line
        code_without_comments.push('\n');
    }

    println!("{}", code_without_comments);

    code_without_comments
}

fn code_chunckenizer(code: String) -> Vec<String> {

    let mut inside_string = false;
    let mut chuncks = Vec::<String>::new();
    let mut buffer = String::new();

    for c in code.chars() {

        match c {
            '"' => {
                buffer.push('"');
                inside_string = !inside_string
            }
            ';' if !inside_string => {
                buffer.push(';');
                chuncks.push(buffer.to_string());
                buffer.clear();
            }
            _ => buffer.push(c),
        }

    }

    if !buffer.is_empty() {
        chuncks.push(buffer.to_string());
        buffer.clear();
    }

    chuncks

}

/// Tokenizer *WIP*
pub fn tokenize(code: String) -> Vec<Token> {
    // println!("{}", code);

    let mut tokens: Vec<Token> = Vec::new();

    let mut buffer: String = String::new();

    let mut inside_string = false;

    let mut inside_type_params = false;

    let mut counter = 0;

    let mut chars = code.chars().peekable();

    while let Some(c) = chars.next() {
        if inside_string {
            match c {
                '"' if !buffer.ends_with('\\') => {
                    inside_string = !inside_string;
                    tokens.push(Token::LiteralString(buffer.clone()));
                    // println!("{buffer}");
                    buffer.clear();
                }
                '"' if buffer.ends_with('\\') => {
                    buffer.pop();
                    buffer.push('"');
                }
                '\\' => {
                    let next_char = check_next_char(&code, &counter);

                    if buffer.ends_with('\\') {
                        buffer.pop();
                        buffer.push('\\');
                    } else {
                        if next_char == '\\' {
                            buffer.push('\\');
                        } else {
                            match next_char {
                                'n' => buffer.push('\n'),
                                't' => buffer.push('\t'),
                                _ => {}
                            }
                            chars.next();
                            counter+=1;
                        }
                    }

                }
                '\n' | '\t' => {

                }
                _ => buffer.push(c),
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
        // println!("Token {}: {:?}", counter, token);
    }

    // println!("{tokens:?}");

    tokens
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
