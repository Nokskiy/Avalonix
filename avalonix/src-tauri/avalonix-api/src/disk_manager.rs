use crate::utils::{OS, current_os};
use std::{
    env::var,
    fs::{self, File},
    path::PathBuf,
};

fn data_ext() -> String {
    String::from(".avalonix_data")
}

fn user_path() -> PathBuf {
    match current_os() {
        OS::Windows => PathBuf::from(var("USERPROFILE").expect("sth wrong with user path")),
        OS::Linux | OS::MacOs => PathBuf::from(var("HOME").expect("sth wrong with user path")),
        _ => panic!("unknown os"),
    }
}

fn check_or_create_dir(path: &PathBuf) {
    if path.exists() {
        if path.is_file() {
            _ = fs::remove_file(&path);
            _ = fs::create_dir_all(&path);
        }
    } else {
        _ = fs::create_dir_all(&path);
    }
}

fn check_or_create_file(path: &PathBuf) {
    if path.exists() {
        if path.is_dir() {
            _ = fs::remove_dir(&path);
            _ = File::create(&path);
        }
    } else {
        _ = File::create(&path);
    }
}

pub fn avalonix_special_folder_path() -> PathBuf {
    let path = user_path().join(".avalonix");
    check_or_create_dir(&path);
    path
}

pub fn hash_path() -> PathBuf {
    let path = avalonix_special_folder_path().join("hash".to_string() + &data_ext());
    check_or_create_file(&path);
    path
}

#[test]
fn test_avalonix_folder_path() {
    println!("{:#?}", avalonix_special_folder_path());
}

#[test]
fn test_hash_folder_path() {
    println!("{:#?}", hash_path());
}
