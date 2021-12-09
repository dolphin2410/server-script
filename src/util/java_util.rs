use std::process::Command;
use regex::Regex;

/// Returns whether the java executable exists. This checks if the `java` command returns something instead of an error.
pub fn java_exists() -> bool {
    if let Ok(_) = Command::new("java").output() {
        true
    } else {
        false
    }
}

/// Finds the `JAVA_HOME` environment variable. Returns `Option::None` if doesn't exist.
pub fn java_home() -> Option<String> {
    if let Ok(data) = std::env::var("JAVA_HOME") {
        Option::Some(data)
    } else {
        Option::None
    }
}

/// Find the java executable. Searches the `java` command, and if it doesn't exist, it will search for the `JAVA_HOME` environment variable. Will panic otherwise.
pub fn find_executable() -> String {
    if java_exists() {
        String::from("java")
    } else if let Some(home) = java_home() {
        home
    } else {
        panic!("No Java Executable Found!")
    }
}

/// Gets the Java version. Regex parsing will be used with the result of `java --version`.
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

/// Checks if the Java version is JDK_9 or up.
pub fn jdk9_up() -> bool {
    java_version().split(".").next().unwrap().parse::<i32>().unwrap() >= 9

}