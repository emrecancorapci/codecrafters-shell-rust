use shell_starter_rust::tokenizer::{Token, Tokenizer};

#[test]
fn hello_world() {
    let test_string = "hello world".to_string();
    let expected_result = vec![
        Token::Value("hello".to_string()),
        Token::Space,
        Token::Value("world".to_string()),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn spacey_hello_world() {
    let test_string = "hello                 world".to_string();
    let expected_result = vec![
        Token::Value("hello".to_string()),
        Token::Space,
        Token::Value("world".to_string()),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn echo_hello_world_single_quote() {
    let test_string = "echo 'example test'".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("example test".to_string(), false),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn echo_hello_world_double_quote() {
    let test_string = "echo \"hello world\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn spacey_echo_hello_world() {
    let test_string = "echo \"hello                   world\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello                   world".to_string(), true),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn double_inside_single_quote() {
    let test_string = "echo '\"hello world\"'".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("\"hello world\"".to_string(), false),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn single_inside_double_quote() {
    let test_string = "echo \"'hello world'\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("'hello world'".to_string(), true),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn single_dash_argument() {
    let test_string = "echo -s 'hello world'".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::Argument("s".to_string(), false),
        Token::Space,
        Token::String("hello world".to_string(), false),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn double_dash_argument() {
    let test_string = "echo --silent 'hello world'".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::Argument("silent".to_string(), true),
        Token::Space,
        Token::String("hello world".to_string(), false),
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn redirector() {
    let test_string = "echo \"hello world\" > \"./hello.md\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }

    assert!(tokenizer.is_redirect() || tokenizer.is_append());

    assert_eq!(
        tokenizer.get_redirection_type(),
        Some(&Token::Redirector(1))
    );

    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_vec_eq(
        tokenizer.get_redirection_tokens().as_ref(),
        &expected_redirection,
    )
}

#[test]
fn redirector_with_number() {
    let test_string = "echo \"hello world\" 2> \"./hello.md\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }

    assert!(tokenizer.is_redirect() || tokenizer.is_append());

    assert_eq!(
        tokenizer.get_redirection_type(),
        Some(&Token::Redirector(2))
    );

    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_vec_eq(
        tokenizer.get_redirection_tokens().as_ref(),
        &expected_redirection,
    )
}

#[test]
fn appender() {
    let test_string = "echo \"hello world\" >> \"./hello.md\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }

    assert!(tokenizer.is_redirect() || tokenizer.is_append());

    assert_eq!(tokenizer.get_redirection_type(), Some(&Token::Appender(1)));

    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_vec_eq(
        tokenizer.get_redirection_tokens().as_ref(),
        &expected_redirection,
    )
}

#[test]
fn appender_with_number() {
    let test_string = "echo \"hello world\" 2>> \"./hello.md\"".to_string();
    let expected_result = vec![
        Token::Value("echo".to_string()),
        Token::Space,
        Token::String("hello world".to_string(), true),
        Token::Space,
    ];

    let mut tokenizer = Tokenizer::new();

    match tokenizer.parse(test_string) {
        Ok(_) => {
            assert_vec_eq(tokenizer.get_tokens_ref(), &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }

    assert!(tokenizer.is_redirect() || tokenizer.is_append());

    assert_eq!(tokenizer.get_redirection_type(), Some(&Token::Appender(2)));

    let expected_redirection = vec![Token::Space, Token::String("./hello.md".to_string(), true)];

    assert_vec_eq(
        tokenizer.get_redirection_tokens().as_ref(),
        &expected_redirection,
    )
}

fn assert_vec_eq<T: std::fmt::Debug + PartialEq>(vec1: &[T], vec2: &[T]) {
    if vec1 != vec2 {
        panic!(
            "Vectors are not equal.\nLeft: {:?}\nRight: {:?}",
            vec1, vec2
        );
    }
}
