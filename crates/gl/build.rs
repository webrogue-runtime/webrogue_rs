fn main() {
    // println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-search=framework={}", "OpenGL");

}
