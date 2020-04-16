fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-search=./workaround");
        println!("cargo:rustc-link-lib=static=workaround");
        println!("cargo:rustc-link-search={}", "./fakenode");
        println!("cargo:rustc-link-lib=node");
        println!("cargo:rustc-link-lib=kernel32");
        // println!(
        //     "cargo:rustc-cdylib-link-arg={}",
        //     r#"C:\Users\amos\AppData\Local\node-gyp\Cache\13.12.0\x64\node.lib"#
        // );
    }

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
        }
    }
}
