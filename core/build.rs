fn main() {
    println!("cargo:rustc-link-search=native=E:/VSProjekte/motion-hid-bridge/libs");
    println!("cargo:rustc-link-lib=ViGEmClient");
}
