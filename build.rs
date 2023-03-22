fn main() {
    println!("cargo:rustc-link-arg=-Tconf/linker.ld");
    println!("cargo:rerun-if-changed=conf/linker.ld");
}
