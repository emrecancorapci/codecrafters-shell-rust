use shell_starter_rust::input_parser::{Input, InputParser};

#[test]
fn hello_world() {
    let test_string = "hello world".to_string();
    let expected_result = vec![
        Input::Command("hello".to_string()),
        Input::Command("world".to_string()),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("hello".to_string()),
        Input::Command("world".to_string()),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::String("example test".to_string(), false),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::String("hello world".to_string(), true),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::String("hello                   world".to_string(), true),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::String("\"hello world\"".to_string(), false),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::String("'hello world'".to_string(), true),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::Argument("s".to_string(), false),
        Input::String("hello world".to_string(), false),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
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
        Input::Command("echo".to_string()),
        Input::Argument("silent".to_string(), true),
        Input::String("hello world".to_string(), false),
    ];

    let mut parser = InputParser::new();

    match parser.parse(test_string) {
        Ok(parsed_vector) => {
            assert_vec_eq(parsed_vector, &expected_result);
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            assert!(false);
        }
    }
}



fn assert_vec_eq<T: std::fmt::Debug + PartialEq>(vec1: &[T], vec2: &[T]) {
    if vec1 != vec2 {
        panic!(
            "Vectors are not equal.\nLeft: {:?}\nRight: {:?}",
            vec1, vec2
        );
    }
}
