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
fn analyze_line<'a>(line: String, in_multi_line_comment: &'a mut bool, in_string_literal: &'a mut bool, string_literal: &'a mut String) -> (&'a mut bool, &'a mut bool, &'a mut String) {
    // Initialize a mutable String to store the current check
    let mut check = String::new();

    // Iterate over each character in the line
    for (_i, c) in line.chars().enumerate() {
        // Check if the character is a delimiter and not in a string literal
        if is_delimiter(c) && !*in_string_literal {
            // Check for single line comment
            if c == '/' && line.chars().next().unwrap() == '/' {
                // If a single line comment is found, break the loop
                break;
            } 
            // Check for multi line comment start
            else if c == '/' && line.chars().next().unwrap() == '*' && !*in_multi_line_comment {
                // If a multi line comment start is found, toggle the in_multi_line_comment flag and return
                *in_multi_line_comment = !*in_multi_line_comment;
                return (in_multi_line_comment, in_string_literal, string_literal);
            } 
            // Check for multi line comment end
            else if c == '*' && line.chars().next().unwrap() == '/' && *in_multi_line_comment { 
                // If a multi line comment end is found, toggle the in_multi_line_comment flag
                *in_multi_line_comment = false;
            } 
            // Check for string literal start
            else if c == '"' {
                // If a string literal start is found, toggle the in_string_literal flag
                *in_string_literal = !*in_string_literal;
                continue;
            } 
            // Check if the character is a symbol and not in a multi line comment
            else if is_symbol(c) && !*in_multi_line_comment {
                // Print the symbol
                println!("{}", c);
            } 
            // Check if the check is a keyword
            else if is_keyword(&check) {
                // Print the keyword
                println!("{} ", check);
                check = String::new();
            } 
            // Check if the check can be parsed as a number
            else if check.parse::<f64>().is_ok() {
                // Print the number
                println!("{} ", check);
                check = String::new();
            } 
            // Check if the check starts with a number
            else if match check.chars().nth(0) {
                Some(c) => c.is_numeric(),
                None => continue
            } {
                // Error if a number starts the identifier
                panic!("Identifier cannot start with number!");
            } 
            // Otherwise, print the identifier
            else {
                println!("{}", check);
                check = String::new();
            }
        } 
        // If in a string literal, append the character to the string_literal
        else if *in_string_literal {
            if c == '"' {
                *in_string_literal = !*in_string_literal;
            } else {
                string_literal.push(c);
            }
        } 
        // Otherwise, append the character to the check
        else {
            check.push(c);
        }
    }

    // Return the updated values
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

    // Initialize variables for the analyze_line function
    let mut res: (&mut bool, &mut bool, &mut String) =
        (&mut false, &mut false, &mut String::new());

    // Analyze each line of the file
    for line in contents.lines() {
        res = analyze_line(line.to_string(), res.0, res.1, res.2);
    }
}