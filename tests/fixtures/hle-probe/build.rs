fn main() {
    println!("cargo:rustc-link-arg-cdylib=/DELAYLOAD:rissing-probe-delayed.dll");
    println!("cargo:rustc-link-lib=delayimp");
}
