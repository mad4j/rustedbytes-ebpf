use std::{
    env,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Options {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Build the eBPF program and the userspace loader
    Build(BuildOptions),
    /// Run the userspace loader (requires root)
    Run(RunOptions),
    /// Run integration tests inside a virtual machine
    TestVm(TestVmOptions),
}

#[derive(Debug, Parser)]
struct BuildOptions {
    /// Build for release
    #[clap(long)]
    release: bool,
}

#[derive(Debug, Parser)]
struct RunOptions {
    /// Network interface to attach the XDP program to
    #[clap(short, long, default_value = "eth0")]
    iface: String,
    /// Log level (e.g. info, debug, warn, error)
    #[clap(long, default_value = "warn")]
    log_level: String,
}

#[derive(Debug, Parser)]
struct TestVmOptions {
    /// Path to the Vagrantfile directory (defaults to test-env/)
    #[clap(long, default_value = "test-env")]
    vm_dir: PathBuf,
}

fn workspace_root() -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .args(["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .expect("failed to run cargo locate-project");
    let cargo_toml =
        String::from_utf8(output.stdout).expect("cargo locate-project output is not UTF-8");
    PathBuf::from(cargo_toml.trim())
        .parent()
        .expect("Cargo.toml has no parent directory")
        .to_owned()
}

fn build(opts: &BuildOptions) -> Result<()> {
    let mut args = vec!["build", "--package", "rustedbytes-ebpf"];
    if opts.release {
        args.push("--release");
    }
    let status = Command::new(env!("CARGO"))
        .args(&args)
        .status()
        .context("failed to run cargo build")?;
    if !status.success() {
        anyhow::bail!("cargo build failed with status: {status}");
    }
    Ok(())
}

fn run(opts: &RunOptions) -> Result<()> {
    // Build first
    build(&BuildOptions { release: true })?;

    let root = workspace_root();
    let binary = root.join("target/release/rustedbytes-ebpf");

    let status = Command::new("sudo")
        .env("RUST_LOG", &opts.log_level)
        .arg(binary)
        .arg("--iface")
        .arg(&opts.iface)
        .status()
        .context("failed to run rustedbytes-ebpf")?;
    if !status.success() {
        anyhow::bail!("rustedbytes-ebpf exited with status: {status}");
    }
    Ok(())
}

fn test_vm(opts: &TestVmOptions) -> Result<()> {
    let root = workspace_root();
    let vm_dir = root.join(&opts.vm_dir);

    // Check Vagrant is available
    which::which("vagrant").context(
        "vagrant not found - install Vagrant from https://www.vagrantup.com/downloads",
    )?;

    // Start the VM
    let status = Command::new("vagrant")
        .arg("up")
        .current_dir(&vm_dir)
        .status()
        .context("failed to start Vagrant VM")?;
    if !status.success() {
        anyhow::bail!("vagrant up failed with status: {status}");
    }

    // Run the tests inside the VM
    let status = Command::new("vagrant")
        .args(["ssh", "-c", "cd /vagrant && cargo test 2>&1"])
        .current_dir(&vm_dir)
        .status()
        .context("failed to run tests inside Vagrant VM")?;

    // Halt the VM after tests
    let _ = Command::new("vagrant")
        .arg("halt")
        .current_dir(&vm_dir)
        .status();

    if !status.success() {
        anyhow::bail!("tests inside VM failed with status: {status}");
    }
    Ok(())
}

fn main() -> Result<()> {
    let opts = Options::parse();
    match opts.command {
        Commands::Build(o) => build(&o),
        Commands::Run(o) => run(&o),
        Commands::TestVm(o) => test_vm(&o),
    }
}
