use std::env;

pub enum OS {
    Windows,
    Linux,
    MacOs,
    Unknown,
}

pub fn current_os() -> OS {
    match env::consts::OS {
        "macos" => OS::MacOs,
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        _ => OS::Unknown,
    }
}
