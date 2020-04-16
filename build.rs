fn main() {
    println!("cargo:rustc-cdylib-link-arg={}", r#"C:\Users\amos\AppData\Local\node-gyp\Cache\13.12.0\x64\node.lib"#);
    println!("cargo:rustc-cdylib-link-arg=-Wl,--undefined");
    println!("cargo:rustc-cdylib-link-arg=-undefined");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}
