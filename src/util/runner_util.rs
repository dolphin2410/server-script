use termcolor::Color;
use crate::config::Configuration;
use crate::util::java_util::jdk9_up;
use crate::util::logger;

/// Generates the default running arguments
pub fn default_args<'a>(jarfile: &'a str, config: &'a Configuration) -> Vec<String> {
    let mut default = vec![
        format!("-Xmx{}G", config.memory).as_str(),
        format!("-Xms{}G", config.memory).as_str(),
        "-XX:+ParallelRefProcEnabled",
        "-XX:MaxGCPauseMillis=200",
        "-XX:+UnlockExperimentalVMOptions",
        "-XX:+DisableExplicitGC",
        "-XX:+AlwaysPreTouch",
        "-XX:G1HeapWastePercent=5",
        "-XX:G1MixedGCCountTarget=4",
        "-XX:G1MixedGCLiveThresholdPercent=90",
        "-XX:G1RSetUpdatingPauseTimePercent=5",
        "-XX:SurvivorRatio=32",
        "-XX:+PerfDisableSharedMem",
        "-XX:MaxTenuringThreshold=1",
        "-Dusing.aikars.flags=https://mcflags.emc.gs",
        "-Daikars.new.flags=true",
        "-Dcom.mojang.eula.agree=true"
    ].into_iter().map(String::from).collect::<Vec<String>>();

    default.append(
        &mut if config.memory < 12 {
            logger::log("Using Aikar's standard memory options", Some(Color::Magenta), None);
            vec![
                "-XX:G1NewSizePercent=30",
                "-XX:G1MaxNewSizePercent=40",
                "-XX:G1HeapRegionSize=8M",
                "-XX:G1ReservePercent=20",
                "-XX:InitiatingHeapOccupancyPercent=15"
            ]
        } else {
            logger::log("Using Aikar's advanced memory options", Some(Color::Magenta), None);
            vec![
                "-XX:G1NewSizePercent=40",
                "-XX:G1MaxNewSizePercent=50",
                "-XX:G1HeapRegionSize=16M",
                "-XX:G1ReservePercent=15",
                "-XX:InitiatingHeapOccupancyPercent=20"
            ]
        }.into_iter().map(String::from).collect::<Vec<String>>()

    );

    println!("Launching Server...");

    if config.debug {
        default.append(
            &mut vec![
                format!("-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address={}{}", if jdk9_up() {
                    "*:"
                } else {
                    ""
                }, config.debug_port)
            ]
        )
    }

    default.append(&mut vec![
        "-jar",
        jarfile,
        "nogui"
    ].into_iter().map(String::from).collect::<Vec<String>>());

    default
}