use std::process::{self, Command};

fn main() {
    let has_git = match Command::new("git").arg("--version").output() {
        Ok(output) => !output.stdout.is_empty(),
        _ => false,
    };
    if !has_git {
        eprintln!("error: perfect6502-sys: Git is required");
        process::exit(1);
    }

    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();

    cc::Build::new()
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-return-type")
        .include("perfect6502")
        .file("perfect6502/netlist_sim.c")
        .file("perfect6502/perfect6502.c")
        .compile("perfect6502");
}
