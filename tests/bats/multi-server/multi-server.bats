#!/usr/bin/env bats

load ../util.bash

project="./tests/bats/multi-server"

@test "Legacy Forge - Test primary and secondary servers stop when idle" {
    # wait for primary lazymc process to start
    echo "Waiting for primary lazymc process to start..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "lazymc-docker-proxy::entrypoint" "Starting lazymc process for group: primary..."

    # wait for secondary lazymc process to start
    echo "Waiting for secondary lazymc process to start..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "lazymc-docker-proxy::entrypoint" "Starting lazymc process for group: secondary..."

    # wait for lazymc to start the primary proxy
    wait_for_formatted_log "lazymc-multi-server" "INFO" "primary::lazymc" "Proxying public 0.0.0.0:25565"

    # wait for lazymc to start the secondary proxy
    wait_for_formatted_log "lazymc-multi-server" "INFO" "secondary::lazymc" "Proxying public 0.0.0.0:25566"

    # wait for lazymc to start the primary server
    echo "Waiting for primary server to start..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "primary::lazymc" "Starting server..."

    # wait for lazymc to start the secondary server
    echo "Waiting for secondary server to start..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "secondary::lazymc" "Starting server..."

    # wait for the primary server to be online
    echo "Waiting for primary server to be online..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "primary::lazymc::monitor" "Server is now online" 300

    #wait for the secondary server to be online
    echo "Waiting for secondary server to be online..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "secondary::lazymc::monitor" "Server is now online" 300

    # wait for the primary server to be ready
    echo "Waiting for primary server to be ready..." >&3
    wait_for_log "primary" "RCON running on 0.0.0.0:25575"

    # wait for the secondary server to be ready
    echo "Waiting for secondary server to be ready..." >&3
    wait_for_log "secondary" "RCON running on 0.0.0.0:25575"

    # wait for the primary server to be idle
    echo "Waiting for primary server to be idle..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "primary::lazymc::monitor" "Server has been idle, sleeping..." 120

    # wait for the secondary server to be idle
    echo "Waiting for secondary server to be idle..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "secondary::lazymc::monitor" "Server has been idle, sleeping..." 120

    # wait for lazymc to put primary to sleep
    echo "Waiting for primary server to be sleeping..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "primary::lazymc::monitor" "Server is now sleeping"

    # wait for lazymc to put secondary to sleep
    echo "Waiting for secondary server to be sleeping..." >&3
    wait_for_formatted_log "lazymc-multi-server" "INFO" "secondary::lazymc::monitor" "Server is now sleeping"
}
