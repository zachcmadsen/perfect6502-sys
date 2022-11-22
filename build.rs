fn main() {
    cc::Build::new()
        .include("perfect6502")
        .file("perfect6502/netlist_sim.c")
        .file("perfect6502/perfect6502.c")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-return-type")
        .compile("perfect6502");
}
