///TODO 
/// - Add alternate symbols for xml
/// - Fix string literal checking
/// - Output to xml file
/// - Add error handling
/// - Clean up code

/// Standard library imports
use std::env;
use std::fs;
use std::io;
use std::io::Write;

/// List of all keywords in the programming language.
const KEYWORD: [&str; 21] = [
    "class", "constructor", "function", "method", "int", "boolean", "char", "void", "var", "static", "field", "let", "do", "if", "else", "while", "return", "true", "false", "null", "this"
];

/// List of all symbols in the programming language.
const SYMBOL: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~'
];

/// Determines whether a character is a delimiter.
///
/// A delimiter is a character that separates different parts of the code, such as symbols, spaces, tabs, or newlines.
///
/// # Arguments
///
/// * `c` - The character to check.
///
/// # Returns
///
/// Returns `true` if the character is a delimiter, `false` otherwise.
fn is_delimiter(c: char) -> bool {
    SYMBOL.contains(&c) || c == ' ' || c == '\n' || c == '\t'
}

/// Determines whether a character is a symbol.
///
/// A symbol is a special character used in the programming language, such as symbols for braces, parentheses, or operators.
///
/// # Arguments
///
/// * `c` - The character to check.
///
/// # Returns
///
/// Returns `true` if the character is a symbol, `false` otherwise.
fn is_symbol(c: char) -> bool {
    SYMBOL.contains(&c)
}

/// Determines whether a word is a keyword.
///
/// A keyword is a reserved word in the programming language, such as "if" or "while".
///
/// # Arguments
///
/// * `word` - The word to check.
///
/// # Returns
///
/// Returns `true` if the word is a keyword, `false` otherwise.
fn is_keyword(word: &str) -> bool {
    KEYWORD.contains(&word)
}

/// Analyzes a line of code and prints out the different elements of the line.
///
/// # Arguments
///
/// * `line` - The line of code to be analyzed.
/// * `in_multi_line_comment` - A boolean indicating whether the analyzer is currently inside a multi-line comment.
/// * `in_string_literal` - A boolean indicating whether the analyzer is currently inside a string literal.
/// * `string_literal` - A mutable String that stores the contents of the current string literal.
///
/// # Returns
///
/// Returns a tuple containing the updated values for `in_multi_line_comment`, `in_string_literal`, and `string_literal`.
fn analyze_line(file: &mut fs::File, line: String, (mut in_multi_line_comment, mut in_string_literal, mut string_literal): (bool, bool, String)) -> (bool, bool, String) {
    // Initialize variables for the analyze_line function
    let mut identifier = String::new();
    let mut prev_char = '\0';
    
    for c in line.chars() {
        // check for single line comment
        if c == '/' && prev_char == '/' && !in_string_literal && !in_multi_line_comment {
            return (in_multi_line_comment, in_string_literal, string_literal);
        } // check for multi line comment 
        else if c == '*' && prev_char == '/' && !in_string_literal && !in_multi_line_comment {
            in_multi_line_comment = true;
        } // check for end of multi line comment 
        else if c == '/' && prev_char == '*' && !in_string_literal && in_multi_line_comment {
            in_multi_line_comment = false;
        } // check for start of string literal  
        else if c == '"' && !in_multi_line_comment && !in_string_literal {
            in_string_literal = true;
        } // check for end of string literal
        else if c == '"' && !in_multi_line_comment && in_string_literal {
            in_string_literal = false;
        } // check for string literal contents
        else if in_string_literal {
            string_literal.push(c);
        } // check for symbol
        else if is_symbol(c) && !in_string_literal && !in_multi_line_comment {
            match c {
                '<' => file.write(b"<symbol> &lt </symbol>").expect("Unable to write to file"),
                '>' => file.write(b"<symbol> &gt </symbol>").expect("Unable to write to file"),
                '&' => file.write(b"<symbol> &amp </symbol>").expect("Unable to write to file"),
                '"' => file.write(b"<symbol> &quot </symbol ").expect("Unable to write to file"),
                _ => file.write(b"<symbol> {c} </symbol>").expect("Unable to write to file"),
            };
        } 
        else if !is_delimiter(c) && !in_string_literal && !in_multi_line_comment {
            identifier.push(c);
        }

        prev_char = c;       
    }

    (in_multi_line_comment, in_string_literal, string_literal)
}

/// Main function to read a file and analyze its contents.
///
/// This function takes command line arguments to determine the file path.
/// If no arguments are provided, it prompts for the file path.
/// If one argument is provided, it assumes it is the file path.
/// If more than one argument is provided, it panics with an error message.
fn main() {
    // Variable to store the file path
    let mut path = String::new();

    // Vector to store the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    match args.len() {
        1 => {
            // If no arguments, ask for file path
            println!("Enter file path: ");
            // Read the file path from stdin
            io::stdin()
                .read_line(&mut path)
                .expect("Failed to read line");
        },
        2 => {
            // If one argument, use it as the file path
            path = args[0].clone();
        },
        _ => {
            // If more than one argument, panic with error message
            panic!("Improper number of arguments!");
        },
    }

    // Print the file path
    println!("{} ", path);

    // Read the contents of the file
    let contents = fs::read_to_string(path.trim())
        .expect("Something went wrong reading the file");

    // Open file to write xml output to
    let mut file = fs::File::open("output.xml").unwrap();

    // Initialize variables for the analyze_line function
    let mut results = (false, false, String::new());

    // Analyze each line of the file
    for line in contents.lines() {
        results = analyze_line(&mut file, line.to_string(), results);
    }
}