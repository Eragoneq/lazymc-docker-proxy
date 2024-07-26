use serde::{Deserialize, Serialize};
use std::env;
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
struct ServerSection {
    address: Option<String>,
    block_banned_ips: Option<bool>,
    command: Option<String>,
    directory: Option<String>,
    drop_banned_ips: Option<bool>,
    forge: Option<bool>,
    freeze_process: Option<bool>,
    probe_on_start: Option<bool>,
    send_proxy_v2: Option<bool>,
    wake_on_crash: Option<bool>,
    wake_on_start: Option<bool>,
    wake_whitelist: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct TimeSection {
    minimum_online_time: Option<i32>,
    sleep_after: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct MotdSection {
    sleeping: Option<String>,
    starting: Option<String>,
    stopping: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RconSection {
    enabled: Option<bool>,
    password: Option<String>,
    port: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct AdvancedSection {
    rewrite_server_properties: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct ConfigSection {
    version: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    advanced: AdvancedSection,
    config: ConfigSection,
    motd: MotdSection,
    rcon: RconSection,
    server: ServerSection,
    time: TimeSection,
}

pub fn generate() {
    let server_section: ServerSection = ServerSection {
        address: env::var("SERVER_ADDRESS")
            .expect("SERVER_ADDRESS is not set")
            .into(),
        directory: Some("/server".to_string()),
        command: Some("lazymc-docker-proxy -c".to_string()),
        freeze_process: env::var("SERVER_FREEZE_PROCESS")
            .ok()
            .map(|x: String| x == "true"),
        wake_on_start: env::var("SERVER_WAKE_ON_START")
            .ok()
            .map(|x: String| x == "true"),
        wake_on_crash: env::var("SERVER_WAKE_ON_CRASH")
            .ok()
            .map(|x: String| x == "true"),
        wake_whitelist: env::var("SERVER_WAKE_WHITELIST")
            .ok()
            .map(|x: String| x == "true"),
        block_banned_ips: env::var("SERVER_BLOCK_BANNED_IPS")
            .ok()
            .map(|x: String| x == "true"),
        drop_banned_ips: env::var("SERVER_DROP_BANNED_IPS")
            .ok()
            .map(|x: String| x == "true"),
        probe_on_start: env::var("SERVER_PROBE_ON_START")
            .ok()
            .map(|x: String| x == "true"),
        forge: env::var("SERVER_FORGE").ok().map(|x: String| x == "true"),
        send_proxy_v2: env::var("SERVER_SEND_PROXY_V2")
            .ok()
            .map(|x: String| x == "true"),
    };

    let time_section: TimeSection = TimeSection {
        sleep_after: env::var("TIME_SLEEP_AFTER")
            .ok()
            .and_then(|x: String| x.parse().ok()),
        minimum_online_time: env::var("TIME_MINIMUM_ONLINE_TIME")
            .ok()
            .and_then(|x: String| x.parse().ok()),
    };

    let motd_section: MotdSection = MotdSection {
        sleeping: env::var("MOTD_SLEEPING").ok(),
        starting: env::var("MOTD_STARTING").ok(),
        stopping: env::var("MOTD_STOPPING").ok(),
    };

    let rcon_section: RconSection = RconSection {
        enabled: env::var("RCON_ENABLED").ok().map(|x: String| x == "true"),
        port: env::var("RCON_PORT")
            .ok()
            .and_then(|x: String| x.parse().ok()),
        password: env::var("RCON_PASSWORD").ok(),
    };

    let advanced_section: AdvancedSection = AdvancedSection {
        rewrite_server_properties: Some(false),
    };

    let config_section: ConfigSection = ConfigSection {
        version: Some("0.2.11".to_string()),
    };

    let config: Config = Config {
        server: server_section,
        time: time_section,
        motd: motd_section,
        rcon: rcon_section,
        advanced: advanced_section,
        config: config_section,
    };

    // Convert the config struct to a toml::Value
    let toml_data: toml::Value =
        toml::Value::try_from(config).expect("Failed to convert to TOML data");

    // Convert the toml::Value to a string
    let toml_string: String = toml::to_string(&toml_data).expect("Failed to serialize TOML data");

    // Path to the output TOML file
    let output_path: &Path = Path::new("lazymc.toml");

    // Write the TOML string to the file
    fs::write(output_path, toml_string).expect("Failed to write TOML file");

    println!("Generated lazymc.toml");
}
