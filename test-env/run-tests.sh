#!/usr/bin/env bash
# run-tests.sh - Build and run integration tests inside the Vagrant VM.
#
# Usage:
#   ./run-tests.sh [--iface <interface>]
#
# This script:
#   1. Starts the Vagrant VM (if not already running)
#   2. Builds the eBPF module inside the VM
#   3. Runs the integration tests inside the VM
#   4. Reports results and halts the VM

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IFACE="${1:-eth0}"

check_vagrant() {
    if ! command -v vagrant &>/dev/null; then
        echo "ERROR: vagrant is not installed."
        echo "       Install Vagrant from https://www.vagrantup.com/downloads"
        exit 1
    fi
}

start_vm() {
    echo "==> Starting Vagrant VM..."
    vagrant up
}

run_tests() {
    echo "==> Running integration tests inside VM..."
    vagrant ssh -c "
        set -euo pipefail
        export PATH='/root/.cargo/bin:\$PATH'
        cd /vagrant
        echo '--- Building eBPF module ---'
        cargo build --package rustedbytes-ebpf
        echo '--- Running integration tests ---'
        cargo test --package rustedbytes-ebpf
        echo '--- All tests passed ---'
    "
}

halt_vm() {
    echo "==> Halting Vagrant VM..."
    vagrant halt
}

main() {
    check_vagrant
    cd "$SCRIPT_DIR"
    start_vm
    if run_tests; then
        echo "SUCCESS: All tests passed."
    else
        echo "FAILURE: Tests failed."
        halt_vm
        exit 1
    fi
    halt_vm
}

main "$@"
