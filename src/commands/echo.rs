pub fn echo(commands: Vec<String>) {
    for command in commands.iter().skip(1) {
        print!("{} ", command);
    }
}