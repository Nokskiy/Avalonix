use colored::Colorize;

pub fn info(msg: &str) {
    println!("{}", "INFO ".green().to_string() + msg);
}

pub fn error(msg: &str) {
    println!("{}", "ERROR ".red().to_string() + msg);
}

pub fn debug(msg: &str) {
    println!("{}", "DEBUG ".blue().to_string() + msg);
}

pub fn fatal(msg: &str) {
    println!("{}", "FATAL ".red().to_string() + msg);
}

pub fn warn(msg: &str) {
    println!("{}", "WARN ".yellow().to_string() + msg);
}
