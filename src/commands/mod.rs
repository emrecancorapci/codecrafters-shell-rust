use std::collections::HashMap;

type ICommand = dyn Fn(Vec<String>);

const SUPPORTED_COMMANDS: [&str; 3] = ["echo", "type", "exit"];

pub fn get_commands() -> HashMap<&'static str, &'static ICommand> {
    let mut map: HashMap<&str, &ICommand> = HashMap::new();
    map.insert("type", &type_);
    map.insert("echo", &echo);
    map.insert("exit", &exit);
    map
}

pub fn echo(inputs: Vec<String>) {
    print!("{}", inputs[1..].join(" "));
    
}

pub fn type_(inputs: Vec<String>) {
    for command in SUPPORTED_COMMANDS.iter() {
        if &inputs[1] == command {
            print!("{} is a shell builtin", command);
            return;
        }
    }

    for path in std::env::var("PATH").unwrap().split(":") {
        let path = format!("{}/{}", path, inputs[1]);

        if std::fs::metadata(&path).is_ok() {
            print!("{} is {}", inputs[1], path);
            return;
        }
    } 

    print!("{} not found", inputs[1]);
}

pub fn exit(inputs: Vec<String>) {
    if inputs[1] == "0" {
        std::process::exit(0);
    } else {
        print!("{}: command not found", inputs[0]);
    }
}
