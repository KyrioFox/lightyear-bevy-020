fn main() {
    println!("cargo:rustc-check-cfg=cfg(docsrs_aeronet)");
}
