mod config;
use config::Config;
use log::Level;
use regex::Regex;
use std::{io::{BufRead, BufReader}, process::{self, exit}, str::FromStr};

use crate::docker;

pub fn run() {
    // Ensure all server containers are stopped before starting
    info!(target: "lazymc-docker-proxy::entrypoint", "Ensuring all server containers are stopped...");
    docker::stop_all_containers();

    let labels_list = docker::get_container_labels();
    let mut configs: Vec<Config> = Vec::new();
    let mut children: Vec<process::Child> = Vec::new();

    for label in labels_list {
        configs.push(Config::from_container_labels(label));
    }

    if configs.is_empty() {
        configs.push(Config::from_env());
    }

    for config in configs {
        let group: String = config.group().into();

        info!(target: "lazymc-docker-proxy::entrypoint", "Starting lazymc process for group: {}...", group.clone());
        let mut child: process::Child = config.start_command()
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| {
                error!(target: "lazymc-docker-proxy::entrypoint", "Failed to start lazymc process for group: {}: {}", group.clone(), err);
                exit(1);
            });

        let mut stdout = child.stdout.take();
        let group_clone = group.clone();
        std::thread::spawn(move || {
            let stdout_reader = BufReader::new(stdout.take().unwrap());
            for line in stdout_reader.lines() {
                wrap_log(&group_clone, line);
            }
        });

        let mut stderr = child.stderr.take();
        std::thread::spawn(move || {
            let stderr_reader = BufReader::new(stderr.take().unwrap());
            for line in stderr_reader.lines() {
                wrap_log(&group.clone(), line)
            }
        });

        children.push(child);
    }

    // If this app receives a signal, stop all server containers
    ctrlc::set_handler(move || {
        info!(target: "lazymc-docker-proxy::entrypoint", "Received exit signal. Stopping all server containers...");
        docker::stop_all_containers();
        exit(0);
    }).unwrap();

    info!(target: "lazymc-docker-proxy::entrypoint", "Setup complete. Waiting for exit signal...");

    // wait indefinitely
    loop {
        std::thread::park();
    }

    
}

fn wrap_log(group: &String, line: Result<String, std::io::Error>) {
    if let Ok(line) = line {
        let regex: Regex = Regex::new(r"(?P<level>[A-Z]+)\s+(?P<target>[a-zA-Z0-9:_-]+)\s+>\s+(?P<message>.+)$").unwrap();
        if let Some(captures) = regex.captures(&line) {
            let level = captures.name("level").unwrap().as_str();
            let target = captures.name("target").unwrap().as_str();
            let message = captures.name("message").unwrap().as_str();

            let wrapped_target = &format!("{}::{}", group, target);
            let log_message = format!("{}", message);
            log!(target: wrapped_target, Level::from_str(level).unwrap_or(Level::Warn), "{}", log_message);
        } else {
            print!("{}", line);
        }
    }
}
