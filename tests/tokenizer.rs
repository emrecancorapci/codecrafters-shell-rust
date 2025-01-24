use std::io::ErrorKind;

use shell_starter_rust::tokenizer::{Token, Tokenizer};

// Basic

#[test]
fn empty_input() {
    let input = "".to_string();
    let expected: Vec<Token> = vec![];

    assert_parsing(input, expected);
}

#[test]
fn basic_command() {
    let input = "hello world".to_string();
    let expected = vec![
        Token::Value("hello".to_string()),
        Token::Space,
        Token::Value("world".to_string()),
    ];

    assert_parsing(input, expected);
}

#[test]
fn multiple_spaces() {
    let input = "hello                 world".to_string();
    let expected = vec![
        Token::Value("hello".to_string()),
        Token::Space,
        Token::Value("world".to_string()),
    ];

    assert_parsing(input, expected);
}

// Quotes

#[test]
fn single_quote() {
    let input = "echo 'example test'".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("example test".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn unclosed_single_quote() {
    let input = "echo 'unclosed single quote".to_string();

    assert_parsing_err(input);
}

#[test]
fn double_quote() {
    let input = "echo \"hello world\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn unclosed_double_quote() {
    let input = "echo \"unclosed double quote".to_string();

    assert_parsing_err(input);
}

#[test]
fn double_quotes_with_escaped_characters() {
    let input = "echo \"escaped \\\"double quotes\\\"\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("escaped \"double quotes\"".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_quotes_with_wide_space() {
    let input = "echo \"hello                   world\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello                   world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn escaped_backslash_in_double_quote() {
    let input = "echo \"hello\\\\world\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello\\world".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_inside_single_quote() {
    let input = "echo '\"hello world\"'".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("\"hello world\"".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn single_inside_double_quote() {
    let input = "echo \"'hello world'\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("'hello world'".to_string(), true),
    ];

    assert_parsing(input, expected);
}

#[test]
fn mixed_quotes_and_arguments() {
    let input = "cmd 'single' \"double\" --arg1 -a".to_string();
    let expected = vec![
        Token::Value("cmd".to_string()),
        Token::Space,
        Token::String("single".to_string(), false),
        Token::Space,
        Token::String("double".to_string(), true),
        Token::Space,
        Token::Argument("arg1".to_string(), true),
        Token::Space,
        Token::Argument("a".to_string(), false),
    ];

    assert_parsing(input, expected);
}

// Arguments

#[test]
fn single_dash_argument() {
    let input = "echo -s 'hello world'".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::Argument("s".to_string(), false),
        Token::Space,
        Token::String("hello world".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn double_dash_argument() {
    let input = "echo --silent 'hello world'".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::Argument("silent".to_string(), true),
        Token::Space,
        Token::String("hello world".to_string(), false),
    ];

    assert_parsing(input, expected);
}

#[test]
fn redirection_operator() {
    let input = "echo \"hello world\" > \"./hello.md\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];
    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_redirection_parsing(input, expected, expected_redirection, Token::Redirector(1));
}

#[test]
fn error_redirection_operator() {
    let input = "echo \"hello world\" 2> \"./hello.md\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];
    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_redirection_parsing(input, expected, expected_redirection, Token::Redirector(2));
}

#[test]
fn redirection_without_target() {
    let input = "echo >".to_string();

    assert_parsing_err(input);
}

#[test]
fn appender() {
    let input = "echo \"hello world\" >> \"./hello.md\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];
    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_redirection_parsing(input, expected, expected_redirection, Token::Appender(1));
}

#[test]
fn appender_with_number() {
    let input = "echo \"hello world\" 2>> \"./hello.md\"".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];
    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_redirection_parsing(input, expected, expected_redirection, Token::Appender(2));
}

#[test]
fn invalid_character() {
    let input = "echo hello @world".to_string();

    assert_parsing_err(input);
}

#[test]
fn mixed_quotes() {
    let input = "echo \"double quotes\" 'single quotes'".to_string();
    let expected = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("double quotes".to_string(), true),
        Token::Space,
        Token::String("single quotes".to_string(), false),
    ];

    assert_parsing(input, expected);
}

fn assert_vec_eq<T: std::fmt::Debug + PartialEq>(vec1: &[T], vec2: &[T]) {
    if vec1 != vec2 {
        panic!(
            "Vectors are not equal.\nLeft: {:?}\nRight: {:?}",
            vec1, vec2
        );
    }
}

fn assert_parsing(input: String, expected: Vec<Token>) -> Tokenizer {
    let mut tokenizer = Tokenizer::new();
    match tokenizer.parse(input) {
        Ok(_) => assert_vec_eq(tokenizer.get_tokens_ref(), &expected),
        Err(err) => panic!("Unexpected error: {}", err),
    }

    tokenizer
}

fn assert_parsing_err(input: String) {
    let mut tokenizer = Tokenizer::new();
    let result = tokenizer.parse(input);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), ErrorKind::InvalidInput);
}

fn assert_redirection_parsing(
    input: String,
    expected: Vec<Token>,
    expected_redirection: Vec<Token>,
    expected_type: Token,
) {
    let mut tokenizer = Tokenizer::new();
    match tokenizer.parse(input) {
        Ok(_) => assert_vec_eq(tokenizer.get_tokens_ref(), &expected),
        Err(err) => panic!("Unexpected error: {}", err),
    }

    assert!(tokenizer.is_redirect() || tokenizer.is_append());
    assert_eq!(tokenizer.get_redirection_type(), Some(&expected_type));

    assert_vec_eq(
        tokenizer.get_redirection_tokens().as_ref(),
        &expected_redirection,
    )
}
