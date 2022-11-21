fn main() {
    cc::Build::new()
        .file("src/netlist_sim.c")
        .file("src/perfect6502.c")
        .include("include")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-return-type")
        .compile("perfect6502");
}
