fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if arch != "x86" {
        eprintln!(
            "error: samp-sdk can be compiled only for x86 target arch (got {}).",
            arch
        );
        panic!();
    }
}
