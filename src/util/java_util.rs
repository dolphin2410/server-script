use std::process::Command;
use regex::Regex;

pub fn java_exists() -> bool {
    if let Ok(_) = Command::new("java").output() {
        true
    } else {
        false
    }
}

pub fn java_home() -> Option<String> {
    if let Ok(data) = std::env::var("JAVA_HOME") {
        Option::Some(data)
    } else {
        Option::None
    }
}

pub fn find_executable() -> String {
    if java_exists() {
        String::from("java")
    } else if let Some(home) = java_home() {
        home
    } else {
        panic!("No Java Executable Found!")
    }
}

pub fn java_version() -> String {
    let ver_text = String::from_utf8(
        Command::new(find_executable())
            .arg("--version")
            .output()
            .unwrap()
            .stdout
    ).unwrap();
    let re = Regex::new(r"(\d+\.).{3}").unwrap();
    let matches = re.find(ver_text.as_str());
    String::from(matches.unwrap().as_str())
}

pub fn jdk9_up() -> bool {
    java_version().split(".").next().unwrap().parse::<i32>().unwrap() >= 9

}