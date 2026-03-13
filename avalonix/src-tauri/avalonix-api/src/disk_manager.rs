use crate::utils::{OS, current_os};
use std::{env::var, path::PathBuf};

fn user_path() -> PathBuf {
    match current_os() {
        OS::Windows => PathBuf::from(var("USERPROFILE").expect("sth wrong with user path")),
        OS::Linux | OS::MacOs => PathBuf::from(var("HOME").expect("sth wrong with user path")),
        _ => panic!("unknown os"),
    }
}

pub fn avalonix_special_folder_path() -> String {
    let path = user_path().join(".avalonix");
    path.to_str().unwrap().to_string()
}

#[test]
fn test_avalonix_folder_path() {
    println!("{:#?}", avalonix_special_folder_path());
}
