fn main() {
    println!(
        "cargo:rustc-link-search={}",
        r#"C:\Users\amos\AppData\Local\node-gyp\Cache\13.12.0\x64"#
    );
    println!("cargo:rustc-link-lib={}", "node");
}
