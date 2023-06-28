fn main() {
    println!("cargo:rerun-if-changed=src/C/gpu_select.c");
    cc::Build::new()
        .file("src/C/gpu_select.c")
        .compile("gpu_select");
}