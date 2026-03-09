use crate::utils::{OS, current_os};
use std::{env::var, path::PathBuf};

pub fn user_path() -> PathBuf {
    match current_os() {
        OS::Windows => PathBuf::from(var("USERPROFILE").expect("sth wrong with user path")),
        OS::Linux | OS::MacOs => PathBuf::from(var("HOME").expect("sth wrong with user path")),
        _ => panic!("unknown os"),
    }
}

#[test]
fn test_user_path() {
    println!("{:#?}", user_path());
}
